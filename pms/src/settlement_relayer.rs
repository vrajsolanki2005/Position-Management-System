use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use crate::models::ModifyAction;
use super::SettlementRelayer;

pub struct DefaultSettlementRelayer {
    rpc_client: solana_client::rpc_client::RpcClient,
}

impl DefaultSettlementRelayer {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_client: solana_client::rpc_client::RpcClient::new(rpc_url),
        }
    }
}

#[async_trait::async_trait]
impl SettlementRelayer for DefaultSettlementRelayer {
    async fn close_position(&self, owner: Pubkey, symbol: &str, exit_price: u64, funding_payment: i64) -> Result<String> {
        let tx_sig = format!("close_{}_{}", symbol, uuid::Uuid::new_v4().to_string()[..8].to_string());
        Ok(tx_sig)
    }

    async fn modify_position(&self, owner: Pubkey, symbol: &str, action: ModifyAction) -> Result<String> {
        let tx_sig = format!("modify_{}_{}", symbol, uuid::Uuid::new_v4().to_string()[..8].to_string());
        Ok(tx_sig)
    }

    async fn liquidate_position(&self, owner: Pubkey, symbol: &str, close_base: u64, mark_price: u64) -> Result<String> {
        let tx_sig = format!("liquidate_{}_{}", symbol, uuid::Uuid::new_v4().to_string()[..8].to_string());
        Ok(tx_sig)
    }
}