/// Acts as the main server that interacts with the front-end to serve
/// whatever my family needs

// Crate imports
use axum::{routing::{get, post}, Router};
use tower_http::cors::CorsLayer;
use std::{net::SocketAddr, sync::{Arc, RwLock}};

// Modules used
mod app_data;
mod user_info;
mod user_funcs;
mod user_session;
mod page_funcs;
mod date;

// Internal imports
use crate::user_funcs::{create_user, login_user, check_valid_key, get_user_data};
use crate::page_funcs::root;
use crate::app_data::AppData;

/// Runs the program:
/// - Sets up a router and binds it to functions that implement specific
///   solutions
/// - Sets up a socket to listen to the client-side requests
#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let app_state = Arc::new(RwLock::new(AppData::load()));
    // Map routes to their respective function
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user))
        .route("/login", post(login_user))
        .route("/get-user-data", post(get_user_data))
        .route("/session-check", post(check_valid_key))
        .with_state(Arc::clone(&app_state))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

