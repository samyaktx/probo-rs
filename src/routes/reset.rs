use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn reset_in_memory(Path(user_id): Path<String>) -> impl IntoResponse  {
    (StatusCode::CREATED, Json(json!({ "msg": format!("User {} created", user_id) })))
}