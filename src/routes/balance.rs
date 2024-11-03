use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use serde::Deserialize;

use crate::engine::inr_balance::InrBalance;
use crate::engine::stock_balance::StockBalance;
use crate::engine::{StockSymbol, UserId};

// user inr_balance
pub async fn get_user_inr_balance(Path(user_id): Path<String>) -> impl IntoResponse {
    let instance = InrBalance::instance();

    if instance.lock().unwrap().user_exists(&user_id) {
        let instance = instance.lock().unwrap();
        let balance = instance.get_balance(&user_id).unwrap().balance;
        let locked = instance.get_balance(&user_id).unwrap().locked;
        (StatusCode::CREATED, Json(json!({ "msg": format!("User balance {} and locked {}", balance, locked) })))
    } else {
        (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": format!("User {} doesn't exists", user_id) })))
    }
}

// return inr_balance
pub async fn get_inr_balance(Path(user_id): Path<String>) -> impl IntoResponse  {
    (StatusCode::CREATED, Json(json!({ "msg": format!("User {} created", user_id) })))
}

pub async fn get_user_stock(Path(user_id): Path<String>) -> impl IntoResponse {
    let instance = StockBalance::instance();
    if instance.lock().unwrap().user_stock_exist(&user_id) {
        let mutex_guard_stock_balance = instance.lock().unwrap();
        let stock_balance = mutex_guard_stock_balance.get_user_stocks(&user_id).unwrap();
        (StatusCode::CREATED, Json(json!({ "msg": format!("User Stock balance {:?}", stock_balance) })))
    } else {
        (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": format!("User {} doesn't have stock balance", user_id) })))
    }
    
}

// user stock_balance

#[derive(Deserialize)]
pub struct StockManager {
    user_id: UserId,
    stock_symbol: StockSymbol,
}
pub async fn get_user_stock_balance(Json(payload): Json<StockManager>) -> impl IntoResponse  {
    let user_id = payload.user_id;
    let stock_symbol = payload.stock_symbol;
    let instance = StockBalance::instance();

    if instance.lock().unwrap().user_stock_exist(&user_id) {
        let instance = instance.lock().unwrap();
        let yes_stock = &instance.get_user_stock_balance(&user_id, &stock_symbol).unwrap().yes;   
        let no_stock = &instance.get_user_stock_balance(&user_id, &stock_symbol).unwrap().no;
        
        let yes_stock_quantity = yes_stock.quantity;
        let yes_stock_locked = yes_stock.locked;
        let no_stock_quantity = no_stock.quantity;
        let no_stock_locked = no_stock.locked;

        // let locked = instance.get_stocks(&user_id).unwrap();
        (StatusCode::CREATED, Json(json!({ 
            "user_id": format!("{}", user_id),
            "stock_symbol": format!("{}", stock_symbol),
            "yes_stock": format!("quantity {} and locked {}", yes_stock_quantity, yes_stock_locked), 
            "no_stock": format!("quantity {} and locked {}", no_stock_quantity, no_stock_locked) 
        })))
    } else {
        (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": format!("User {} doesn't have stocks", user_id) })))
    }
}

// return stock_balance
pub async fn get_stock_balance(Path(user_id): Path<String>) -> impl IntoResponse  {
    (StatusCode::CREATED, Json(json!({ "msg": format!("User {} created", user_id) })))
}