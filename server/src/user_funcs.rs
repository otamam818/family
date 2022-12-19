use std::fs;

/// The file that controls all the functions defined by the user
use axum::{response::IntoResponse, Json, http::StatusCode};
use crate::{user_info::{CreateUser, User, Gender}, date::Date};

// TODO: Connect the payload to a SignUpData instead of a UserAuth
/// Creates a new user from the UserAuth data
pub async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    let secret_code = match fs::read_to_string("data/secret_code.txt") {
        Err(message) => panic!("Error: {}", message),
        Ok(text) => text
    };

    match secret_code.cmp(&payload.secret_code) {
        std::cmp::Ordering::Equal => {},
        _ => return (StatusCode::NOT_FOUND, Err("Invalid code"))
    };

    let user = User {
        username: payload.username,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        gender: Gender::MALE,
        place_of_birth: "Germany".to_string(),
        birthday: Date::new(21, 12, 1890).unwrap()
    };

    (StatusCode::CREATED, Ok(Json(user)))
}

