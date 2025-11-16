use std::sync::Arc;
use anyhow::Result;
use solana_sdk::{pubkey::Pubkey, system_program, signature::Signer};
use anchor_client::Program;

use crate::{models::{OpenPositionInput, ModifyAction, PositionView, Side, PositionState}, solana::{client::SolanaCtx, pda}, db::repo::PgRepo};
use super::{margin::MarginCalculator, pnl::PnLTracker};

#[derive(Clone)]
pub struct PositionManager {
    sol: Arc<SolanaCtx>,
    repo: PgRepo,
    margin: MarginCalculator,
    pnl: PnLTracker,
    program_id: Pubkey,
}

impl PositionManager {
    pub fn new(sol: SolanaCtx, repo: PgRepo, margin: MarginCalculator, pnl: PnLTracker, program_id: Pubkey) -> Self {
        Self { sol: Arc::new(sol), repo, margin, pnl, program_id }
    }

    // Opens a position by sending the Anchor instruction "open_position"
    pub async fn open_position(&self, input: OpenPositionInput) -> Result<Pubkey> {
        let owner = self.sol.payer.pubkey();
        let (user_pda, _ub) = pda::user_pda(&self.program_id, &owner);
        let (position_pda, _pb) = pda::position_pda(&self.program_id, &owner, &input.symbol);
        let (vault_pda, _vb) = pda::vault_pda(&self.program_id, &input.quote_mint);
        let (vault_auth_pda, _ab) = pda::vault_authority_pda(&self.program_id);

        // Build and send tx through Anchor client with IDL loaded (recommend embedding IDL JSON)
        // Pseudo:
        // self.sol.program
        //   .request()
        //   .accounts(position_manager::accounts::OpenPosition { ... })
        //   .args(position_manager::instruction::OpenPosition { ... })
        //   .send()?;

        // For skeleton purposes, we'll just record intent in DB, actual tx code to be filled once IDL is wired.
        self.repo.insert_position_open_intent(&owner, &position_pda, &input).await?;

        Ok(position_pda)
    }

    pub async fn modify_position(&self, owner: Pubkey, symbol: &str, action: ModifyAction) -> Result<()> {
        let (position_pda, _pb) = pda::position_pda(&self.program_id, &owner, symbol);

        // TODO: send "modify_position" instruction via Anchor client
        self.repo.insert_position_modify_intent(&owner, &position_pda, &action).await?;
        Ok(())
    }

    pub async fn close_position(&self, owner: Pubkey, symbol: &str, exit_price: u64, funding_payment: i64) -> Result<()> {
        let (position_pda, _pb) = pda::position_pda(&self.program_id, &owner, symbol);

        // TODO: send "close_position" instruction via Anchor client
        self.repo.insert_position_close_intent(&owner, &position_pda, exit_price, funding_payment).await?;
        Ok(())
    }

    // Query on-chain position (via IDL) or from DB snapshot
    pub async fn get_position(&self, owner: Pubkey, symbol: &str) -> Result<Option<PositionView>> {
        let (pda, _) = pda::position_pda(&self.program_id, &owner, symbol);
        // If you embed the IDL, you can do:
        // let pos: OnChainPosition = self.sol.program.account(pda)?;
        // Convert to PositionView and return.
        self.repo.fetch_position_view(&pda).await
    }

    pub async fn list_positions_by_user(&self, owner: Pubkey) -> Result<Vec<PositionView>> {
        self.repo.fetch_positions_by_owner(&owner).await
    }
}