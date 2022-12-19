/// Acts as the main server that interacts with the front-end to serve
/// whatever my family needs

// Crate imports
use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;

// Modules used
mod user_info;
mod user_funcs;
mod page_funcs;
mod date;

// Internal imports
use crate::user_funcs::create_user;
use crate::page_funcs::root;

/// Runs the program:
/// - Sets up a router and binds it to functions that implement specific
///   solutions
/// - Sets up a socket to listen to the client-side requests
#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    // Map routes to their respective function
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user));

    let addr = SocketAddr::from(([192, 168, 1, 147], 8000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

