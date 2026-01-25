Smart Contract Documentation
-Accounts
Position (PDA: ["position", owner, symbol])
owner: Pubkey
symbol: String (<=16)
side: Long|Short
size: u64
entry_price: u64 (1e6)
margin: u64 (quote)
leverage: u16
unrealized_pnl: i64
realized_pnl: i64
funding_accrued: i64
liquidation_price: u64
last_update: i64
bump: u8
UserAccount (PDA: ["user", owner])
owner, total_collateral, locked_collateral, total_pnl, position_count, bump
Vault (SPL Token PDA: ["vault", quote_mint])
Token account holding locked margin for this mint
VaultAuthority (PDA: ["vault_authority"])
Authority over vault token accounts (signer for outgoing transfers)

-Instructions
open_position(symbol, side, size, leverage, entry_price)
Validates leverage tier, IM
Transfers IM from user ATA to program vault
Creates Position, emits PositionOpened
modify_position(ModifyKind)
IncreaseSize{ add_size, price, add_margin }:
Optional margin transfer in
Leverage/tier checks, weighted entry update
DecreaseSize{ reduce_size, price }:
Realize PnL proportionally
AddMargin{ amount }:
Transfer in, update margin
RemoveMargin{ amount, price }:
Check post-removal MR >= mmr; transfer out
Emits PositionModified
close_position(exit_price, funding_payment)
Realize PnL; payout = max(margin + realized − funding, 0)
Transfers payout to user; closes Position; emits PositionClosed
Optional extension (keeper):
liquidate_position(max_close_base, mark_price): partial/full close with penalty to insurance fund

Example (TypeScript)
await program.methods
  .openPosition("BTC-PERP", { long: {} }, new BN(1000), 100, new BN(30_000_000))
  .accounts({...})
  .rpc();

-Security considerations
Ownership checks: position.owner == signer; user.owner == signer
PDA seeds are validated; vault transfers out signed by vault_authority PDA only
Integer-only math with checked ops; require! guards for div by zero and overflow
Enforce tier max leverage and size; MR guard on remove margin
Atomic state updates per instruction; no partial writes
Oracle integration (when added): validate confidence bands / staleness (off-chain mock during tests)

-Testing strategy
Unit (on-chain): IM/MM calculations, liq price, uPnL/realized logic, MR guard on remove
Integration (TS): open → increase → decrease → remove margin → close; 1000x edge cases
Fuzz: random sizes/prices/leverage to assert no overflow/panic
Parity: backend fixed-point vs on-chain for random corpus
