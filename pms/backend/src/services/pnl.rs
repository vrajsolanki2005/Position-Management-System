#[derive(Debug, Clone, Default)]
pub struct PnLTracker;

impl PnLTracker {
    // Average entry over multiple fills: sum(price * qty) / sum(qty)
    pub fn calculate_average_entry_price(&self, trades: &[(f64, f64)]) -> f64 {
        let (total_value, total_size) = trades.iter().fold((0.0, 0.0), |(v, s), (price, qty)| (v + price * qty, s + qty));
        if total_size == 0.0 { 0.0 } else { total_value / total_size }
    }

    pub fn calculate_unrealized_pnl(&self, is_long: bool, size: f64, mark_price: f64, entry_price: f64) -> f64 {
        if is_long { size * (mark_price - entry_price) } else { size * (entry_price - mark_price) }
    }
}
