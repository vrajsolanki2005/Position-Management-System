use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct FundingRate {
    pub rate: i64,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct MarginRequirements {
    pub initial_margin: u64,
    pub maintenance_margin: u64,
    pub liquidation_threshold: u64,
}

pub fn calculate_mark_price(oracle_price: u64, funding_impact: i64) -> u64 {
    ((oracle_price as i128 + funding_impact as i128) as u64).max(1)
}

pub fn calculate_initial_margin(notional: u64, im_rate: u64) -> u64 {
    notional * im_rate / 10000
}

pub fn calculate_maintenance_margin(notional: u64, mm_rate: u64) -> u64 {
    notional * mm_rate / 10000
}

pub fn calculate_unrealized_pnl(entry_price: u64, mark_price: u64, size: u64, is_long: bool) -> i64 {
    let price_diff = mark_price as i64 - entry_price as i64;
    let pnl = price_diff * size as i64 / entry_price as i64;
    if is_long { pnl } else { -pnl }
}

pub fn calculate_liquidation_price(entry_price: u64, margin: u64, size: u64, mm_rate: u64, is_long: bool) -> u64 {
    let maintenance_margin = calculate_maintenance_margin(size * entry_price / 1000000, mm_rate);
    let max_loss = margin.saturating_sub(maintenance_margin);
    
    if is_long {
        entry_price.saturating_sub(max_loss * entry_price / (size * entry_price / 1000000))
    } else {
        entry_price + (max_loss * entry_price / (size * entry_price / 1000000))
    }
}

pub fn derive_position_pda(program_id: &Pubkey, owner: &Pubkey, symbol: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"position", owner.as_ref(), symbol.as_bytes()],
        program_id
    )
}

pub fn derive_user_pda(program_id: &Pubkey, owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"user", owner.as_ref()], program_id)
}

pub fn calculate_funding_payment(position_size: u64, funding_rate: i64, hours: u64) -> i64 {
    (position_size as i64 * funding_rate * hours as i64) / (10000 * 8)
}