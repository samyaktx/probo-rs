use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::engine::orderbook::OrderBook;

pub async fn get_orderbook() -> impl IntoResponse  {
    let orderbook_instance = OrderBook::instance();
    let orderbook_guard = orderbook_instance.lock().unwrap();
    let orderbook = orderbook_guard.get_orderbook();

    (StatusCode::CREATED, Json(json!({ "msg": format!("{:?}", orderbook) })))
}