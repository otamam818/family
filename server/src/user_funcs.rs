/// The file that controls all the functions defined by the user
use std::fs;
use axum::{response::IntoResponse, Json, http::StatusCode};
use crate::{user_info::{CreateUser, User, Gender}, date::Date};

// TODO: Connect the payload to a SignUpData instead of a UserAuth
/// Creates a new user from the UserAuth data
pub async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    // The assumption is that this is run from the "server" folder
    let secret_code = fs::read_to_string("data/secret_code.txt")
        .expect("File should exist, no matter what");

    // Remove the EOF character from the file
    let secret_code = secret_code[0..secret_code.len()-1].to_string();

    // Check if the secret code matches with the one in the server's
    // filesystem
    match secret_code.cmp(&payload.secret_code) {
        std::cmp::Ordering::Equal => {},
        _ => return (
            StatusCode::UNAUTHORIZED,
            Err(format!("Invalid code: {}", &payload.secret_code))
        )
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

