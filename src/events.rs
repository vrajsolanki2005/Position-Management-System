use anchor_lang::prelude::*;
use crate::state::accounts::Side;

#[event]
pub struct PositionOpened {
    pub owner: Pubkey,
    pub symbol: String,
    pub side: Side,
    pub size: u64,
    pub leverage: u16,
    pub entry_price: u64,
    pub initial_margin: u64,
    pub liquidation_price: u64,
}

#[event]
pub struct PositionModified {
    pub owner: Pubkey,
    pub symbol: String,
    pub size: u64,
    pub margin: u64,
    pub leverage: u16,
    pub price: u64,
    pub unrealized_pnl: i64,
    pub liquidation_price: u64,
}

#[event]
pub struct PositionClosed {
    pub owner: Pubkey,
    pub symbol: String,
    pub size_closed: u64,
    pub exit_price: u64,
    pub realized_pnl: i64,
    pub payout: u64,
}