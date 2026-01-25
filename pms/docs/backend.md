Backend Service Documentation
-Module architecture
services/manager.rs: submits open/modify/close TXs (via anchor-client), fetches accounts, reconciles DB
services/margin.rs, pnl.rs: math utilities (switch to fixed-point for parity)
services/monitor.rs: periodic mark price pulls, MR computing, alerts, snapshots
services/oracle.rs: PriceOracle trait + MockOracle; plug real Pyth reader later
db/repo.rs: typed queries for positions, events, snapshots, alerts
api/http.rs: REST endpoints
api/ws.rs: WebSocket hub for real-time streams
solana/{client.rs, pda.rs}: Anchor client + PDA helpers

-API specifications
POST /positions/open
Body: { symbol, side: "Long"|"Short", size, leverage, entry_price, margin_token_account, quote_mint }
200: { position_pda, signature? }
PUT /positions/:id/modify
Body: { type: "increase"|"decrease"|"add_margin"|"remove_margin", ... }
200: { ok: true, signature? }
DELETE /positions/:id/close
Body: { exit_price, funding_payment }
200: { ok: true, signature?, payout? }
GET /positions/:id
200: { position: PositionView|null }
GET /users/:owner/positions
200: { positions: PositionView[] }
WebSocket /ws?streams=positions,pnl,alerts,events
positions.update, pnl.update, alerts.margin, position.event messages (JSON)
Database schema documentation

-Core tables (schema perp)
markets(symbol, quote_mint, price_scale, im_rate_ppm, mm_rate_ppm, ...)
users(owner, total_collateral, total_realized_pnl, ...)
positions(pda, owner, symbol, side, size_base, entry_price, margin, leverage, unrealized_pnl, realized_pnl, liquidation_price, state, opened_at, updated_at, closed_at, last_slot, last_signature)
position_modifications(id, ts, slot, signature, position_pda, kind, base_delta, margin_delta, price, fee_paid, funding_paid, realized_pnl_delta, …)
pnl_snapshots(id, bucket_start, granularity, owner, symbol, position_pda, mark_price, unrealized_pnl, realized_pnl_cum, funding_cum, equity, margin_ratio)
user_daily_stats(owner, day, trades_count, gross_volume_quote, fees_paid_quote, realized_pnl_quote, funding_paid_quote, liquidations_count, max_leverage_used, win_trades, loss_trades)
user_risk_metrics(owner, computed_at, total_notional, total_equity, im_req, mm_req, margin_ratio, positions_at_risk, worst_liq_distance, risk_score)

-Configuration
.env
RPC_URL, WS_URL, KEYPAIR_PATH, PROGRAM_ID
DATABASE_URL
HTTP_ADDR (default 0.0.0.0:8080)
PRICE_ORACLE_SOURCE (mock/pyth:SYMBOL)
RISK_ALERT_THRESHOLD (e.g., 0.15)

-Deployment (local)
solana-test-validator --reset
anchor build && anchor deploy
docker run -p 5432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=perp -d postgres:15
cargo run (backend auto-applies schema.sql and starts API)
anchor test (on-chain E2E via TS)

-Deployment (dev/prod)
Use a managed Postgres
Run backend as a service (systemd/docker/k8s)
Set RPC_URL to a reliable RPC (dedicated endpoint)
Configure Prometheus metrics + alerts for MR and ops error rates