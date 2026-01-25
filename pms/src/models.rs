use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct PositionView {
    pub pda: Pubkey,
    pub owner: Pubkey,
    pub symbol: String,
    pub size: u64,
    pub collateral: u64,
    pub entry_price: u64,
}

#[derive(Debug, Clone)]
pub enum ModifyAction {
    IncreaseSize(u64),
    DecreaseSize(u64),
    AddCollateral(u64),
    RemoveCollateral(u64),
}