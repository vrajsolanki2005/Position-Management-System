use crate::models::PositionView;
use solana_sdk::pubkey::Pubkey;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum OrderType {
    StopLoss { trigger_price: u64 },
    TakeProfit { target_price: u64 },
    TrailingStop { trail_amount: u64, peak_price: u64 },
}

#[derive(Debug, Clone)]
pub struct AdvancedOrder {
    pub id: String,
    pub owner: Pubkey,
    pub symbol: String,
    pub order_type: OrderType,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

pub struct OrderManager {
    orders: Vec<AdvancedOrder>,
}

impl OrderManager {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn add_order(&mut self, order: AdvancedOrder) {
        self.orders.push(order);
    }

    pub fn check_triggers(&mut self, symbol: &str, current_price: u64) -> Vec<AdvancedOrder> {
        let mut triggered = Vec::new();
        
        for order in &mut self.orders {
            if !order.is_active || order.symbol != symbol {
                continue;
            }

            let should_trigger = match &mut order.order_type {
                OrderType::StopLoss { trigger_price } => current_price <= *trigger_price,
                OrderType::TakeProfit { target_price } => current_price >= *target_price,
                OrderType::TrailingStop { trail_amount, peak_price } => {
                    if current_price > *peak_price {
                        *peak_price = current_price;
                    }
                    current_price <= peak_price.saturating_sub(*trail_amount)
                }
            };

            if should_trigger {
                order.is_active = false;
                triggered.push(order.clone());
            }
        }

        triggered
    }
}

pub struct HedgeDetector;

impl HedgeDetector {
    pub fn detect_hedges(positions: &[PositionView]) -> Vec<(usize, usize)> {
        let mut hedges = Vec::new();
        
        for (i, pos1) in positions.iter().enumerate() {
            for (j, pos2) in positions.iter().enumerate().skip(i + 1) {
                if pos1.owner == pos2.owner && 
                   pos1.symbol == pos2.symbol && 
                   Self::is_opposite_direction(pos1, pos2) {
                    hedges.push((i, j));
                }
            }
        }
        
        hedges
    }

    fn is_opposite_direction(pos1: &PositionView, pos2: &PositionView) -> bool {
        // Simplified: assume size sign indicates direction
        (pos1.size as i64) * (pos2.size as i64) < 0
    }
}