Risk Management Guide
-Monitoring
The monitor pulls mark prices (oracle) and recomputes for every open position:
uPnL = size × (mark − entry) for Long (negative if mark < entry)
MR = (margin + uPnL) / (size × mark)
It emits:
pnl.update on every tick (throttled)
alerts.margin when MR < threshold (e.g., 12–20%)
It stores pnl_snapshots for hourly/daily analytics.

-Margin call process
Warning thresholds:
MR < warn1 (e.g., 20%): user notification
MR < warn2 (e.g., 15%): strong alert; suggest adding margin or reducing size
If MR < maintenance_rate:
Position is eligible for liquidation (keeper action)

-Liquidation triggers
Trigger condition (theoretical): MR <= mmr
Practical process:
Backend/keeper detects MR breach
Keeper submits liquidate_position (partial close) or a forced close flow (if implemented) at oracle mark/twap
Apply penalty to insurance fund
Repeat partial liquidation until MR > mmr+buffer or size=0

-Protocol safety mechanisms
Isolated margin per position
Leverage tiers caps reduce tail risk for large notional
Integer math with strict overflow guards
Oracle checks (staleness, confidence) when integrated
Event-sourced DB + reconciler to detect state drift and re-sync
Bounded keeper actions and penalties flowing to insurance fund
Withdraw guard: remove_margin forbidden if post-withdraw MR < mmr
