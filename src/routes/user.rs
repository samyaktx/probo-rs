use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::engine::inr_balance::InrBalance;

// use crate::actions::user::create_new_user;
// pub async fn create_user_id(Path(user_id): Path<String>) -> impl IntoResponse  {
//     let result = create_new_user(&user_id); 
//     match result {
//         Ok(_) => (StatusCode::CREATED, Json(json!({ "msg": format!("User {} created", user_id) }))),
//         Err(e) => (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": e })))
//     }
// }

pub async fn create_user_id(Path(user_id): Path<String>) -> impl IntoResponse {
    let instance = InrBalance::instance();

    if instance.lock().unwrap().user_exists(&user_id) {
        (StatusCode::NOT_ACCEPTABLE, Json(json!({ "msg": format!("User {} already exists", user_id) })))
    } else {
        let mut instance = instance.lock().unwrap();
        instance.add_user(&user_id);

        (StatusCode::CREATED, Json(json!({ "msg": format!("User {} created", user_id) })))
    }
}