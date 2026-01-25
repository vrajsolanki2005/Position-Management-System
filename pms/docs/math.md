Mathematical Formulas
-Notation
size: base quantity (integer)
price: quote per base (integer, 1e6 scale)
notional = size × price
leverage = notional / margin
tier.mm_rate, tier.im_rate: maintenance/initial margin rates (scaled)
Side: Long (+), Short (−)

-Initial margin (IM)
IM = notional / leverage
Example: size=1000, price=30_000_000 (30k), lev=100
notional = 1000 × 30_000_000 = 30_000_000_000
IM = 30_000_000_000 / 100 = 300_000_000

-Maintenance margin (MM)
MM = notional × maintenance_margin_rate
Example: mmr = 0.005 (0.5%)
MM = 30_000_000_000 × 0.005 = 150_000_000

-Unrealized PnL (uPnL)
Long: uPnL = size × (mark − entry)
Short: uPnL = size × (entry − mark)
Example (long): size=1000, entry=30_000_000, mark=30_500_000
uPnL = 1000 × (500_000) = 500_000_000

-Margin ratio (MR)
MR = (margin + uPnL) / (size × mark)
Example: margin=300_000_000, uPnL=500_000_000, mark=30_500_000
position_value = 1000 × 30_500_000 = 30_500_000_000
MR = (300_000_000 + 500_000_000) / 30_500_000_000 ≈ 0.02623 (2.623%)

-Liquidation price 
Let RATE_SCALE = 1e6. mmr is in the same scale.
Long:
P_liq = [(size × entry − margin) × RATE_SCALE] / [size × (RATE_SCALE − mmr)]
Short:
P_liq = [(margin + size × entry) × RATE_SCALE] / [size × (RATE_SCALE + mmr)]
Note: These formulas assume fees/funding folded into margin or uPnL when checking MR.

-Realized PnL
On reduction by reduce_size:
realized = reduce_size × (mark − entry) for Long
realized = reduce_size × (entry − mark) for Short
On full close: same with reduce_size = size

-Funding
Keep cumulative funding per base (quote per 1 base, integer scale).
Per interval:
delta_funding = clamp(rate, ±cap) × dt × price_basis
cum_funding_per_base += delta_funding
Per position settlement:
funding_pnl = base_qty × (cum_now − last_cum) / SCALE
Apply to collateral (or realized bucket), then set last_cum = cum_now