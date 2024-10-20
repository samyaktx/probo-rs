use axum::{routing::{get, post}, Router};

mod user;

use user::create_userx;

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/", get(hello_world))
        .route("/user/create/:userId", post(create_userx))
}

pub async fn hello_world() -> String {
    "hello, world".to_owned()
}