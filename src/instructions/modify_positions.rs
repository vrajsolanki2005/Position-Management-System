use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::PerpError;
use crate::events::PositionModified;
use crate::math::*;
use crate::state::accounts::*;
use crate::tiers::get_leverage_tier;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ModifyKind {
    IncreaseSize { add_size: u64, price: u64, add_margin: u64 },
    DecreaseSize { reduce_size: u64, price: u64 },
    AddMargin { amount: u64 },
    RemoveMargin { amount: u64, price: u64 },
}

pub fn handler(ctx: Context<ModifyPosition>, action: ModifyKind) -> Result<()> {
    match action {
        ModifyKind::IncreaseSize { add_size, price, add_margin } => {
            require!(add_size > 0, PerpError::InvalidSize);

            if add_margin > 0 {
                token::transfer(ctx.accounts.transfer_to_vault_ctx(), add_margin)?;
                ctx.accounts.user.total_collateral = ctx.accounts.user.total_collateral.checked_add(add_margin).ok_or(PerpError::Overflow)?;
                ctx.accounts.user.locked_collateral = ctx.accounts.user.locked_collateral.checked_add(add_margin).ok_or(PerpError::Overflow)?;
                ctx.accounts.position.margin = ctx.accounts.position.margin.checked_add(add_margin).ok_or(PerpError::Overflow)?;
            }

            let pos = &mut ctx.accounts.position;

            let new_size = pos.size.checked_add(add_size).ok_or(PerpError::Overflow)?;
            let new_notional = mul_u128(new_size as u128, price as u128)?;
            let notional_u64 = u128_to_u64(new_notional)?;
            let tier = get_leverage_tier(pos.leverage, notional_u64)?;

            // leverage check: notional / margin <= pos.leverage
            let lev_now = div_u128(new_notional, pos.margin as u128)?;
            require!(lev_now <= pos.leverage as u128, PerpError::InsufficientMarginForIncrease);
            require!(pos.leverage <= tier.max_leverage, PerpError::LeverageExceeded);

            // weighted avg entry
            if price > 0 {
                let old_notional = mul_u128(pos.size as u128, pos.entry_price as u128)?;
                let add_notional = mul_u128(add_size as u128, price as u128)?;
                let total = add_u128(old_notional, add_notional)?;
                let new_entry = div_u128(total, new_size as u128)?;
                pos.entry_price = u128_to_u64(new_entry)?;
            }

            pos.size = new_size;

            let upnl = calc_unrealized_pnl(pos.side, pos.size, pos.entry_price, price)?;
            pos.unrealized_pnl = i128_to_i64(upnl)?;
            pos.liquidation_price = crate::math::calc_liquidation_price(pos.side, pos.size, pos.entry_price, pos.margin, tier.maintenance_margin_rate)?;
            pos.last_update = Clock::get()?.unix_timestamp;

            emit!(PositionModified {
                owner: pos.owner,
                symbol: pos.symbol.clone(),
                size: pos.size,
                margin: pos.margin,
                leverage: pos.leverage,
                price,
                unrealized_pnl: pos.unrealized_pnl,
                liquidation_price: pos.liquidation_price,
            });
        }

        ModifyKind::DecreaseSize { reduce_size, price } => {
            require!(reduce_size > 0 && reduce_size <= ctx.accounts.position.size, PerpError::InvalidSize);

            let realized = calc_realized_pnl_partial(ctx.accounts.position.side, reduce_size, ctx.accounts.position.entry_price, price)?;
            ctx.accounts.position.realized_pnl = ctx.accounts.position.realized_pnl.checked_add(i128_to_i64(realized)?).ok_or(PerpError::Overflow)?;
            ctx.accounts.position.size = ctx.accounts.position.size.checked_sub(reduce_size).ok_or(PerpError::Overflow)?;

            let upnl = calc_unrealized_pnl(ctx.accounts.position.side, ctx.accounts.position.size, ctx.accounts.position.entry_price, price)?;
            ctx.accounts.position.unrealized_pnl = i128_to_i64(upnl)?;
            let new_notional_u64 = u128_to_u64(mul_u128(ctx.accounts.position.size as u128, price as u128)?)?;
            let tier = get_leverage_tier(ctx.accounts.position.leverage, new_notional_u64)?;
            ctx.accounts.position.liquidation_price = crate::math::calc_liquidation_price(ctx.accounts.position.side, ctx.accounts.position.size, ctx.accounts.position.entry_price, ctx.accounts.position.margin, tier.maintenance_margin_rate)?;
            ctx.accounts.position.last_update = Clock::get()?.unix_timestamp;

            emit!(PositionModified {
                owner: ctx.accounts.position.owner,
                symbol: ctx.accounts.position.symbol.clone(),
                size: ctx.accounts.position.size,
                margin: ctx.accounts.position.margin,
                leverage: ctx.accounts.position.leverage,
                price,
                unrealized_pnl: ctx.accounts.position.unrealized_pnl,
                liquidation_price: ctx.accounts.position.liquidation_price,
            });
        }

        ModifyKind::AddMargin { amount } => {
            require!(amount > 0, PerpError::InvalidAmount);
            token::transfer(ctx.accounts.transfer_to_vault_ctx(), amount)?;
            ctx.accounts.user.total_collateral = ctx.accounts.user.total_collateral.checked_add(amount).ok_or(PerpError::Overflow)?;
            ctx.accounts.user.locked_collateral = ctx.accounts.user.locked_collateral.checked_add(amount).ok_or(PerpError::Overflow)?;
            ctx.accounts.position.margin = ctx.accounts.position.margin.checked_add(amount).ok_or(PerpError::Overflow)?;
            ctx.accounts.position.last_update = Clock::get()?.unix_timestamp;

            let pos = &ctx.accounts.position;

            emit!(PositionModified {
                owner: pos.owner,
                symbol: pos.symbol.clone(),
                size: pos.size,
                margin: pos.margin,
                leverage: pos.leverage,
                price: pos.entry_price,
                unrealized_pnl: pos.unrealized_pnl,
                liquidation_price: pos.liquidation_price,
            });
        }

        ModifyKind::RemoveMargin { amount, price } => {
            require!(amount > 0 && amount <= ctx.accounts.position.margin, PerpError::InvalidAmount);

            let notional = mul_u128(ctx.accounts.position.size as u128, price as u128)?;
            let upnl = calc_unrealized_pnl(ctx.accounts.position.side, ctx.accounts.position.size, ctx.accounts.position.entry_price, price)?;
            let new_margin = (ctx.accounts.position.margin as i128).checked_sub(amount as i128).ok_or(PerpError::Overflow)?;
            let mr_num = (new_margin as i128).checked_add(upnl as i128).ok_or(PerpError::Overflow)?;
            let mr_den = notional as i128;
            require!(mr_den > 0, PerpError::InvalidState);

            let tier = get_leverage_tier(ctx.accounts.position.leverage, u128_to_u64(notional)?)?;
            let lhs = mul_i128_i128(mr_num as i128, crate::constants::RATE_SCALE as i128)?;
            let rhs = mul_u128_u64(notional, tier.maintenance_margin_rate as u64)?;
            require!(lhs >= rhs as i128, PerpError::MaintenanceBreach);

            // transfer out from vault to user (PDA signer)
            let signer_seeds: &[&[u8]] = &[b"vault_authority", &[ctx.accounts.vault_authority.bump]];
            token::transfer(
                ctx.accounts.transfer_from_vault_ctx().with_signer(&[signer_seeds]),
                amount,
            )?;

            ctx.accounts.user.locked_collateral = ctx.accounts.user.locked_collateral.checked_sub(amount).ok_or(PerpError::Overflow)?;
            ctx.accounts.user.total_collateral = ctx.accounts.user.total_collateral.checked_sub(amount).ok_or(PerpError::Overflow)?;
            ctx.accounts.position.margin = ctx.accounts.position.margin.checked_sub(amount).ok_or(PerpError::Overflow)?;
            ctx.accounts.position.unrealized_pnl = i128_to_i64(upnl)?;
            ctx.accounts.position.liquidation_price = crate::math::calc_liquidation_price(ctx.accounts.position.side, ctx.accounts.position.size, ctx.accounts.position.entry_price, ctx.accounts.position.margin, tier.maintenance_margin_rate)?;
            ctx.accounts.position.last_update = Clock::get()?.unix_timestamp;

            let pos = &ctx.accounts.position;

            emit!(PositionModified {
                owner: pos.owner,
                symbol: pos.symbol.clone(),
                size: pos.size,
                margin: pos.margin,
                leverage: pos.leverage,
                price,
                unrealized_pnl: pos.unrealized_pnl,
                liquidation_price: pos.liquidation_price,
            });
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct ModifyPosition<'info> {
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

impl<'info> ModifyPosition<'info> {
    pub fn transfer_to_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_quote_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn transfer_from_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_quote_ata.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}