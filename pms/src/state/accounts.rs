use anchor_lang::prelude::*;
use crate::constants::MAX_SYMBOL_LEN;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Long,
    Short,
}

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub symbol: String,      // <= 16 chars
    pub side: Side,          // Long or Short
    pub size: u64,           // base units
    pub entry_price: u64,    // quote per base
    pub margin: u64,         // locked collateral (quote)
    pub leverage: u16,       // 1..=1000
    pub unrealized_pnl: i64, // snapshot
    pub realized_pnl: i64,   // quote
    pub funding_accrued: i64,// quote
    pub liquidation_price: u64,
    pub last_update: i64,
    pub bump: u8,
}

impl Position {
    pub fn space(max_symbol: usize) -> usize {
        8  // discriminator
        + 32 // owner
        + 4 + max_symbol // symbol string
        + 1  // side
        + 8  // size
        + 8  // entry_price
        + 8  // margin
        + 2  // leverage
        + 8  // unrealized_pnl
        + 8  // realized_pnl
        + 8  // funding_accrued
        + 8  // liquidation_price
        + 8  // last_update
        + 1  // bump
        + 32 // extra padding room
    }
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub total_collateral: u64,
    pub locked_collateral: u64,
    pub total_pnl: i64,
    pub position_count: u32,
    pub bump: u8,
}

impl UserAccount {
    pub const SPACE: usize = 8  // disc
        + 32 + 8 + 8 + 8 + 4 + 1
        + 16; // padding
}

#[account]
pub struct VaultAuthority {
    pub bump: u8,
}