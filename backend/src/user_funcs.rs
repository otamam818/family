/// The file that controls all the functions defined by the user
use axum::{response::IntoResponse, Json, http::StatusCode};
use crate::user_info::{UserAuth, User};

/// Creates a new user from the UserAuth data
pub async fn create_user(Json(payload): Json<UserAuth>,) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username
    };

    (StatusCode::CREATED, Json(user))
}

