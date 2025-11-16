# Initial Setup Guide

## Environment Requirements

### 1. Rust Development Environment
- ✅ **Rust 1.75+**: Currently installed 1.91.0
- ✅ **Async/await support**: Available in current version
- ❌ **Anchor framework 0.29+**: Not installed
- ❌ **Solana CLI tools**: Not installed
- ❌ **PostgreSQL**: Not installed

## Installation Commands

### Install Anchor Framework
```bash
npm install -g @coral-xyz/anchor-cli@0.29.0
```

### Install Solana CLI
```bash
# Windows
curl https://release.solana.com/v1.16.0/solana-install-init-x86_64-pc-windows-msvc.exe --output solana-install-init.exe
solana-install-init.exe v1.16.0
```

### Install PostgreSQL
```bash
# Download from https://www.postgresql.org/download/windows/
# Or use chocolatey:
choco install postgresql
```

## Core Concepts Implementation

### Perpetual Futures Mechanics
```rust
pub struct FundingRate {
    pub rate: i64,        // 8-hour funding rate in basis points
    pub timestamp: i64,   // Unix timestamp
}

pub fn calculate_mark_price(oracle_price: u64, funding_impact: i64) -> u64 {
    ((oracle_price as i128 + funding_impact as i128) as u64).max(1)
}
```

### Margin Calculations
```rust
pub fn calculate_initial_margin(notional: u64, im_rate: u64) -> u64 {
    notional * im_rate / 10000 // im_rate in basis points
}

pub fn calculate_maintenance_margin(notional: u64, mm_rate: u64) -> u64 {
    notional * mm_rate / 10000 // mm_rate in basis points
}
```

### PnL Calculations
```rust
pub fn calculate_unrealized_pnl(
    entry_price: u64,
    mark_price: u64,
    size: u64,
    is_long: bool
) -> i64 {
    let price_diff = mark_price as i64 - entry_price as i64;
    let pnl = price_diff * size as i64 / entry_price as i64;
    if is_long { pnl } else { -pnl }
}
```

### Solana Account Rent & PDA
```rust
use solana_sdk::{pubkey::Pubkey, rent::Rent, sysvar::Sysvar};

pub fn calculate_rent_exempt_balance(data_len: usize) -> u64 {
    Rent::get().unwrap().minimum_balance(data_len)
}

pub fn derive_position_pda(program_id: &Pubkey, owner: &Pubkey, symbol: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"position", owner.as_ref(), symbol.as_bytes()],
        program_id
    )
}
```

## Current Project Status
- ✅ Rust environment ready
- ✅ Trading engine implemented
- ✅ Database schema created
- ✅ Test scenarios completed
- ❌ Missing: Anchor, Solana CLI, PostgreSQL installation