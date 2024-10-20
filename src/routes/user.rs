use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::db::UserData;
pub async fn create_userx(Path(user_id): Path<String>) -> impl IntoResponse {
    UserData::new(user_id.clone());
    (StatusCode::CREATED, Json(json!({ "msg": format!("User {} created", user_id) })))
}
