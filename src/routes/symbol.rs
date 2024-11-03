use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::engine::orderbook::OrderBook;

pub async fn create_stock_symbol(Path(stock_symbol): Path<String>) -> impl IntoResponse  {
    let orderbook_instance = OrderBook::instance();
    let mut mutex_guard_orderbook = orderbook_instance.lock().unwrap();
    let create_market = mutex_guard_orderbook.create_market(stock_symbol.clone());
    if create_market.is_ok() {
        (StatusCode::CREATED, Json(json!({ "msg": format!("Stock Symbol {} created", stock_symbol) })))
    } else {
        (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": format!("{}", create_market.err().unwrap()) })))
    }
}