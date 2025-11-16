use crate::errors::PerpError;

#[derive(Clone, Copy)]
pub struct LeverageTierInt {
    pub max_leverage: u16,
    pub initial_margin_rate: u64,    // scaled by 1e6
    pub maintenance_margin_rate: u64,// scaled by 1e6
    pub max_position_size: u64,      // in quote units
}

pub const LEVERAGE_TIERS: [LeverageTierInt; 5] = [
    LeverageTierInt { max_leverage: 20, initial_margin_rate: 50_000, maintenance_margin_rate: 25_000, max_position_size: u64::MAX },
    LeverageTierInt { max_leverage: 50, initial_margin_rate: 20_000, maintenance_margin_rate: 10_000, max_position_size: 100_000 },
    LeverageTierInt { max_leverage: 100, initial_margin_rate: 10_000, maintenance_margin_rate: 5_000, max_position_size: 50_000 },
    LeverageTierInt { max_leverage: 500, initial_margin_rate: 5_000, maintenance_margin_rate: 2_500, max_position_size: 20_000 },
    LeverageTierInt { max_leverage: 1000, initial_margin_rate: 2_000, maintenance_margin_rate: 1_000, max_position_size: 5_000 },
];

pub fn get_leverage_tier(leverage: u16, pos_size_quote: u64) -> Result<LeverageTierInt, anchor_lang::prelude::Error> {
    for t in LEVERAGE_TIERS.iter() {
        if leverage <= t.max_leverage && pos_size_quote <= t.max_position_size {
            return Ok(*t);
        }
    }
    Err(PerpError::LeverageExceeded.into())
}