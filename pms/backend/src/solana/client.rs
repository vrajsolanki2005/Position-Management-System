use std::sync::Arc;
use anyhow::Result;
use anchor_client::{Client, Cluster, Program};
use solana_sdk::{signature::{read_keypair_file, Keypair}, commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::config::Config;

pub struct SolanaCtx {
    pub program_id: Pubkey,
    pub payer: Arc<Keypair>,
    pub rpc_url: String,
}

impl SolanaCtx {
    pub async fn new(cfg: &Config) -> Result<Self> {
        let payer = Arc::new(read_keypair_file(&cfg.keypair_path).map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?);
        Ok(Self { 
            program_id: cfg.program_id,
            payer,
            rpc_url: cfg.rpc_url.clone(),
        })
    }
}