use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::PerpError;
use crate::events::PositionClosed;
use crate::math::*;
use crate::state::accounts::*;

pub fn handler(
    ctx: Context<ClosePosition>,
    exit_price: u64,
    funding_payment: i64,
) -> Result<()> {
    let pnl = calc_realized_pnl_full(ctx.accounts.position.side, ctx.accounts.position.size, ctx.accounts.position.entry_price, exit_price)?;
    let pnl_i64 = i128_to_i64(pnl)?;
    let net_pnl = pnl_i64.checked_sub(funding_payment).ok_or(PerpError::Overflow)?;
    ctx.accounts.position.funding_accrued = ctx.accounts.position.funding_accrued.checked_add(-funding_payment).ok_or(PerpError::Overflow)?;

    let payout_i128 = (ctx.accounts.position.margin as i128 + net_pnl as i128).max(0);
    let payout_u64 = i128_to_u64(payout_i128)?;

    if payout_u64 > 0 {
        let signer_seeds: &[&[u8]] = &[b"vault_authority", &[ctx.accounts.vault_authority.bump]];
        token::transfer(
            ctx.accounts.transfer_from_vault_ctx().with_signer(&[signer_seeds]),
            payout_u64,
        )?;
    }

    ctx.accounts.user.locked_collateral = ctx.accounts.user.locked_collateral.checked_sub(ctx.accounts.position.margin).ok_or(PerpError::Overflow)?;
    ctx.accounts.user.total_collateral = ctx.accounts.user.total_collateral.checked_sub(ctx.accounts.position.margin).ok_or(PerpError::Overflow)?;
    ctx.accounts.user.total_pnl = ctx.accounts.user.total_pnl.checked_add(net_pnl).ok_or(PerpError::Overflow)?;
    ctx.accounts.user.position_count = ctx.accounts.user.position_count.saturating_sub(1);

    emit!(PositionClosed {
        owner: ctx.accounts.position.owner,
        symbol: ctx.accounts.position.symbol.clone(),
        size_closed: ctx.accounts.position.size,
        exit_price,
        realized_pnl: net_pnl,
        payout: payout_u64,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump = user.bump,
        constraint = user.owner == owner.key()
    )]
    pub user: Account<'info, UserAccount>,

    #[account(
        mut,
        close = owner,
        seeds = [b"position", owner.key().as_ref(), position.symbol.as_bytes()],
        bump = position.bump,
        constraint = position.owner == owner.key()
    )]
    pub position: Account<'info, Position>,

    pub quote_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_quote_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", quote_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_authority"],
        bump = vault_authority.bump
    )]
    pub vault_authority: Account<'info, VaultAuthority>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ClosePosition<'info> {
    pub fn transfer_from_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_quote_ata.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}