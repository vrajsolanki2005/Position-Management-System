use axum::{routing::{get, post, put, delete}, Router, extract::{State, Path}, Json};
use std::{net::SocketAddr};
use serde::Deserialize;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

use crate::{services::manager::PositionManager, db::repo::PgRepo, models::{OpenPositionInput, ModifyAction, PositionView}};

#[derive(Clone)]
pub struct AppState {
    pub manager: PositionManager,
    pub repo: PgRepo,
}

pub async fn start_http_server(addr: String, manager: PositionManager, repo: PgRepo) -> Result<()> {
    let state = AppState { manager, repo };
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/positions/open", post(open_position))
        .route("/positions/:id/modify", put(modify_position))
        .route("/positions/:id/close", delete(close_position))
        .route("/positions/:id", get(get_position))
        .route("/users/:owner/positions", get(list_positions))
        .with_state(state);

    let addr: SocketAddr = addr.parse()?;
    tracing::info!("HTTP listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn open_position(State(st): State<AppState>, Json(input): Json<OpenPositionInput>) -> Json<serde_json::Value> {
    let pda = st.manager.open_position(input).await.unwrap();
    Json(serde_json::json!({ "position_pda": pda.to_string(), "signature": null }))
}

#[derive(Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
enum ModifyReq {
    Increase { add_size: u64, price: u64, add_margin: u64 },
    Decrease { reduce_size: u64, price: u64 },
    AddMargin { amount: u64 },
    RemoveMargin { amount: u64, price: u64 },
}

async fn modify_position(State(st): State<AppState>, Path(id): Path<String>, Json(req): Json<ModifyReq>) -> Json<serde_json::Value> {
    // Look up owner+symbol from DB using PDA, then call manager.modify_position
    let pda = id.parse::<Pubkey>().unwrap();
    let pos = st.repo.fetch_position_view(&pda).await.unwrap().expect("position not found");
    let owner = pos.owner;
    let symbol = pos.symbol.clone();

    let action = match req {
        ModifyReq::Increase{ add_size, price, add_margin } => ModifyAction::IncreaseSize{ add_size, price, add_margin },
        ModifyReq::Decrease{ reduce_size, price } => ModifyAction::DecreaseSize{ reduce_size, price },
        ModifyReq::AddMargin{ amount } => ModifyAction::AddMargin{ amount },
        ModifyReq::RemoveMargin{ amount, price } => ModifyAction::RemoveMargin{ amount, price },
    };

    st.manager.modify_position(owner, &symbol, action).await.unwrap();
    Json(serde_json::json!({ "ok": true, "signature": null }))
}

#[derive(Deserialize)]
struct CloseReq { exit_price: u64, funding_payment: i64 }

async fn close_position(State(st): State<AppState>, Path(id): Path<String>, Json(req): Json<CloseReq>) -> Json<serde_json::Value> {
    let pda = id.parse::<Pubkey>().unwrap();
    let pos = st.repo.fetch_position_view(&pda).await.unwrap().expect("position not found");
    st.manager.close_position(pos.owner, &pos.symbol, req.exit_price, req.funding_payment).await.unwrap();
    Json(serde_json::json!({ "ok": true, "signature": null, "payout": null }))
}

async fn get_position(State(st): State<AppState>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let pda = id.parse::<Pubkey>().unwrap();
    let pos: Option<PositionView> = st.repo.fetch_position_view(&pda).await.unwrap();
    Json(serde_json::json!({ "position": pos }))
}

async fn list_positions(State(st): State<AppState>, Path(owner): Path<String>) -> Json<serde_json::Value> {
    let owner = owner.parse::<Pubkey>().unwrap();
    let res = st.manager.list_positions_by_user(owner).await.unwrap_or_default();
    Json(serde_json::json!({ "positions": res }))
}