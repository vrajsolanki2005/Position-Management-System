# 🎮 Interactive Demo Guide

## Run Interactive Demo
```bash
cargo run --bin interactive_demo
```

## What You Can Customize

### 1. User Setup
- **Username**: Enter your preferred trader name
- **Default**: demo_trader

### 2. Trading Parameters
- **Trading Pair**: BTC-USD, ETH-USD, SOL-USD, etc.
- **Entry Price**: Any price (e.g., 45000 for BTC)
- **Position Size**: Amount in USD (e.g., 50000)
- **Leverage**: 1x to 20x multiplier
- **Side**: LONG or SHORT position

### 3. Risk Management
- **Stop-Loss**: Percentage below/above entry
- **Take-Profit**: Target profit percentage

### 4. Market Simulation
- **Real-time Price Updates**: Enter new market prices
- **Order Triggers**: Watch stop-loss/take-profit execute
- **PnL Calculation**: See profits/losses in real-time

## Example Session

```
Trading pair: BTC-USD
Entry price: 45000
Position size: 50000
Leverage: 10x
Side: LONG

Stop-loss: 7% (triggers at $41,850)
Take-profit: 10% (triggers at $49,500)

Market prices to try:
- 46000 (small profit)
- 47500 (good profit) 
- 49600 (take-profit triggered!)
```

## Video Recording Tips

### For Live Demo:
1. **Prepare values beforehand**:
   - Symbol: BTC-USD
   - Entry: 45000
   - Size: 50000
   - Leverage: 10x
   - Side: LONG

2. **Market simulation**:
   - Start: 46000 (+$2,222 PnL)
   - Next: 47500 (+$5,556 PnL)
   - Final: 49600 (Take-profit triggered!)

3. **Talk while typing**:
   - "Let me enter a BTC long position..."
   - "I'll use 10x leverage for this demo..."
   - "Now let's simulate market movement..."

### Benefits:
- ✅ **Your own values** - personalized demo
- ✅ **Real calculations** - actual PnL math
- ✅ **Interactive** - engaging for viewers
- ✅ **Flexible** - adapt during recording
- ✅ **Professional** - shows system capabilities

## Quick Commands
```bash
# Interactive demo (your input)
cargo run --bin interactive_demo

# Pre-scripted demo (automatic)
cargo run --bin live_demo

# Example trading scenarios
cargo run --example sol_short_15x
```