use std::sync::Arc;
use anyhow::Result;
use anchor_client::{Client, Cluster, Program};
use solana_sdk::{signature::{read_keypair_file, Keypair}, commitment_config::CommitmentConfig};

use crate::config::Config;

pub struct SolanaCtx {
    pub program: Program<Arc<Keypair>>,
    pub payer: Arc<Keypair>,
}

impl SolanaCtx {
    pub async fn new(cfg: &Config) -> Result<Self> {
        let payer = Arc::new(read_keypair_file(&cfg.keypair_path).map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?);
        let cluster = Cluster::Custom(cfg.rpc_url.clone(), cfg.ws_url.clone());
        let client = Client::new_with_options(cluster, payer.clone(), CommitmentConfig::processed());
        let program = client.program(cfg.program_id)?;
        Ok(Self { program, payer })
    }
}