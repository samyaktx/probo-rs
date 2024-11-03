use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

use crate::engine::inr_balance::InrBalance;
use crate::engine::UserId;

#[derive(Deserialize)]
pub struct Onramp {
    user_id: UserId,
    amount: u64,
}

pub async fn onramp_amount(Json(payload): Json<Onramp>) -> impl IntoResponse  {
    let user_id = payload.user_id;
    let amount = payload.amount;

    let instance = InrBalance::instance();

    if instance.lock().unwrap().user_exists(&user_id) {
        let mut instance = instance.lock().unwrap();
        instance.onramp_balance(&user_id, amount).unwrap();
        let balance_x = instance.get_balance(&user_id).unwrap().balance;
        (StatusCode::CREATED, Json(json!({ "msg": format!("User balance {}", balance_x) })))
    } else {
        (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": format!("User {} doesn't exists", user_id) })))
    }
}