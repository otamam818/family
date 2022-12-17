use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::Path};
use serde_json::json;

use std::net::SocketAddr;

mod user_info;

use crate::user_info::{CreateUser, User};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user))
        .route("/:name", get(json_hello));

    let addr = SocketAddr::from(([192, 168, 1, 147], 8000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, There!"
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({"message": hello + greeting })))
}

async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username
    };

    (StatusCode::CREATED, Json(user))
}

