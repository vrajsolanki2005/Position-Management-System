System Architecture
Units and scaling
All prices and rates are fixed-point integers on-chain.
Price scale: 1e6 (e.g., 30_000.000000 USD → 30_000_000).
Size: integer base units (you define base precision off-chain).
Notional = size × price.
Rates (IM/MM): stored as ppm (parts per million) in code paths that need scaling; tier table is integer-scaled or floating off-chain; on-chain uses integer rate scale.

Component interaction-Flow Diagram is attched ./architecture_fd

Position lifecycle- flow diagram is attached ./state_fd

Data flows

-Open
Client → Backend → Anchor TX open_position
On-chain: IM check, lock margin, create Position PDA, emit PositionOpened
Backend subscribes events, fetches account, upserts DB, broadcasts WS positions.update
-Modify
Client → Backend → Anchor TX modify_position (increase/decrease/add/remove)
On-chain: checks (IM/MM), adjust state, emit PositionModified
Backend updates DB and WS
-Close
Client/keeper → Backend → Anchor TX close_position
On-chain: realize PnL, transfer payout, close account, emit PositionClosed
Backend finalizes DB state and WS
