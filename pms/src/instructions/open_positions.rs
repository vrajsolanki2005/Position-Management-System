use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::PerpError;
use crate::events::PositionOpened;
use crate::math::*;
use crate::state::accounts::*;
use crate::tiers::{get_leverage_tier};

pub fn handler(
    ctx: Context<OpenPosition>,
    symbol: String,
    side: Side,
    size: u64,
    leverage: u16,
    entry_price: u64,
) -> Result<()> {
    require!(size > 0, PerpError::InvalidSize);
    require!(leverage >= MIN_LEVERAGE && leverage <= MAX_LEVERAGE, PerpError::InvalidLeverage);
    require!(symbol.len() <= MAX_SYMBOL_LEN, PerpError::SymbolTooLong);

    let notional = mul_u128(size as u128, entry_price as u128)?;
    let notional_u64 = u128_to_u64(notional)?;
    let tier = get_leverage_tier(leverage, notional_u64)?;

    let im = div_u128(notional, leverage as u128)?;
    let im_u64 = u128_to_u64(im)?;

    // lock initial margin
    token::transfer(
        ctx.accounts.transfer_to_vault_ctx(),
        im_u64,
    )?;

    // init or update user
    let user = &mut ctx.accounts.user;
    if user.owner == Pubkey::default() {
        user.owner = ctx.accounts.owner.key();
        user.total_collateral = 0;
        user.locked_collateral = 0;
        user.total_pnl = 0;
        user.position_count = 0;
        user.bump = ctx.bumps.user;
    }
    user.total_collateral = user.total_collateral.checked_add(im_u64).ok_or(PerpError::Overflow)?;
    user.locked_collateral = user.locked_collateral.checked_add(im_u64).ok_or(PerpError::Overflow)?;
    user.position_count = user.position_count.checked_add(1).ok_or(PerpError::Overflow)?;

    // create position
    let pos = &mut ctx.accounts.position;
    pos.owner = ctx.accounts.owner.key();
    pos.symbol = symbol.clone();
    pos.side = side;
    pos.size = size;
    pos.entry_price = entry_price;
    pos.margin = im_u64;
    pos.leverage = leverage;
    pos.unrealized_pnl = 0;
    pos.realized_pnl = 0;
    pos.funding_accrued = 0;
    pos.liquidation_price = crate::math::calc_liquidation_price(side, size, entry_price, im_u64, tier.maintenance_margin_rate)?;
    pos.last_update = Clock::get()?.unix_timestamp;
    pos.bump = ctx.bumps.position;

    emit!(PositionOpened {
        owner: pos.owner,
        symbol,
        side,
        size,
        leverage,
        entry_price,
        initial_margin: im_u64,
        liquidation_price: pos.liquidation_price,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(symbol: String)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        space = UserAccount::SPACE
    )]
    pub user: Account<'info, UserAccount>,

    #[account(
        init,
        payer = owner,
        seeds = [b"position", owner.key().as_ref(), symbol.as_bytes()],
        bump,
        space = Position::space(MAX_SYMBOL_LEN)
    )]
    pub position: Account<'info, Position>,

    pub quote_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_quote_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"vault", quote_mint.key().as_ref()],
        bump,
        token::mint = quote_mint,
        token::authority = vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"vault_authority"],
        bump,
        space = 8 + 1
    )]
    pub vault_authority: Account<'info, VaultAuthority>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> OpenPosition<'info> {
    pub fn transfer_to_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_quote_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}