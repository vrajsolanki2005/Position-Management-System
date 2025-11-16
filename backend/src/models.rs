use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side { Long, Short }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionState { Opening, Open, Modifying, Closing, Closed, Liquidating }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionView {
    pub owner: Pubkey,
    pub symbol: String,
    pub side: Side,
    pub size: u64,
    pub entry_price: u64,
    pub margin: u64,
    pub leverage: u16,
    pub unrealized_pnl: i64,
    pub realized_pnl: i64,
    pub liquidation_price: u64,
    pub last_update: DateTime<Utc>,
    pub state: PositionState,
    pub pda: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPositionInput {
    pub symbol: String,
    pub side: Side,
    pub size: u64,
    pub leverage: u16,
    pub entry_price: u64, // or you’ll fetch oracle on-chain
    pub margin_token_account: Pubkey, // user's USDC ATA
    pub quote_mint: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifyAction {
    IncreaseSize { add_size: u64, price: u64, add_margin: u64 },
    DecreaseSize { reduce_size: u64, price: u64 },
    AddMargin { amount: u64 },
    RemoveMargin { amount: u64, price: u64 },
}