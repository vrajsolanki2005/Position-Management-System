use axum::{extract::{Query, State, WebSocketUpgrade}, response::IntoResponse};
use futures::{StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct WsHub {
    pub positions_tx: broadcast::Sender<serde_json::Value>,
    pub pnl_tx: broadcast::Sender<serde_json::Value>,
    pub alerts_tx: broadcast::Sender<serde_json::Value>,
    pub events_tx: broadcast::Sender<serde_json::Value>,
}
impl WsHub {
    pub fn new() -> Self {
        let (a,_)=broadcast::channel(1024);
        let (b,_)=broadcast::channel(1024);
        let (c,_)=broadcast::channel(1024);
        let (d,_)=broadcast::channel(1024);
        Self { positions_tx:a, pnl_tx:b, alerts_tx:c, events_tx:d }
    }
}

#[derive(Deserialize)]
pub struct WsParams {
    owner: Option<String>,
    position: Option<String>,
    streams: Option<String>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<WsHub>,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        let streams = params.streams.clone().unwrap_or_else(|| "positions,pnl,alerts,events".into());
        let subs: Vec<&str> = streams.split(',').map(|s| s.trim()).collect();

        let mut pos_rx = if subs.contains(&"positions") { Some(hub.positions_tx.subscribe()) } else { None };
        let mut pnl_rx = if subs.contains(&"pnl") { Some(hub.pnl_tx.subscribe()) } else { None };
        let mut alerts_rx = if subs.contains(&"alerts") { Some(hub.alerts_tx.subscribe()) } else { None };
        let mut events_rx = if subs.contains(&"events") { Some(hub.events_tx.subscribe()) } else { None };

        loop {
            tokio::select! {
                biased;

                // You can filter by owner/position on publish or here on receive by inspecting message.data
                msg = async {
                    match pos_rx.as_mut() {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if pos_rx.is_some() => {
                    if let Some(Ok(msg)) = msg {
                        let _ = socket.send(axum::extract::ws::Message::Text(msg.to_string())).await;
                    }
                },
                msg = async {
                    match pnl_rx.as_mut() {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if pnl_rx.is_some() => {
                    if let Some(Ok(msg)) = msg {
                        let _ = socket.send(axum::extract::ws::Message::Text(msg.to_string())).await;
                    }
                },
                msg = async {
                    match alerts_rx.as_mut() {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if alerts_rx.is_some() => {
                    if let Some(Ok(msg)) = msg {
                        let _ = socket.send(axum::extract::ws::Message::Text(msg.to_string())).await;
                    }
                },
                msg = async {
                    match events_rx.as_mut() {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if events_rx.is_some() => {
                    if let Some(Ok(msg)) = msg {
                        let _ = socket.send(axum::extract::ws::Message::Text(msg.to_string())).await;
                    }
                },
                else => break,
            }
        }
    })
}