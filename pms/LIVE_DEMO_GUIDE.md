# 🎬 Live Demonstration Guide

## Quick Start
```bash
cargo run --bin live_demo
```

## Demo Flow (8 sections)

### 1. System Initialization
- Shows all components coming online
- **Press ENTER** to continue to next section

### 2. User Configuration  
- Sets up Premium tier user
- Shows leverage and position limits

### 3. Position Opening
- Real-time risk validation
- Opens BTC long position with 10x leverage
- Shows all safety checks

### 4. Advanced Orders
- Adds stop-loss at $42,000
- Adds take-profit at $49,500

### 5. Market Updates
- Simulates real-time price movements
- Shows PnL calculations
- Triggers take-profit order

### 6. Performance Analytics
- Displays trading metrics
- Shows ROI and Sharpe ratio

### 7. Blockchain Verification
- Shows Solana transaction details
- Confirms on-chain execution

### 8. System Health
- Final status check
- All components operational

## Recording Tips

### Before Starting:
```bash
# Test the demo works
cargo run --bin live_demo

# Have these commands ready:
cargo run --example sol_short_15x
cargo test --lib
```

### During Recording:
- **Speak while demo runs** - explain what's happening
- **Use ENTER key** to control pacing
- **Zoom in** on terminal for better visibility
- **Highlight key numbers** with cursor

### Key Talking Points:
1. **"This shows real-time risk validation"**
2. **"Notice the leverage limits based on user tier"**
3. **"The system automatically calculates PnL"**
4. **"Advanced orders trigger based on market conditions"**
5. **"Everything is recorded on Solana blockchain"**

## Alternative Demos

### Quick Example Run:
```bash
cargo run --example sol_short_15x
```

### Test Suite:
```bash
cargo test
```

### Show Code:
- `src/trading_engine.rs`
- `src/risk_manager.rs`
- `src/solana_trade.rs`

## Video Structure

1. **Start with live demo** (5 minutes)
2. **Show source code** (2 minutes)  
3. **Run tests** (1 minute)
4. **Wrap up** (1 minute)

Total: ~9 minutes