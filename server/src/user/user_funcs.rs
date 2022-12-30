/// The file that controls all the functions defined by the user
use std::{fs, sync::{RwLock, Arc}};
use axum::{response::IntoResponse, Json, http::StatusCode, extract::State};
use serde_json::json;

use crate::{
    user::{CreateUser, User, UserAuth},
    date::Date, app_data::AppData
};

// Constants
const USR_PATH: &'static str = "data/info";
const DIR_PATH: &'static str = "data/dirs";

/// Creates a new user from the CreateUser data
#[axum_macros::debug_handler]
pub async fn create_user(
    State(shared_state): State<Arc<RwLock<AppData>>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    const PATH_SECRET_FILE: &'static str = "data/secret_code.txt";

    // The assumption is that this is run from the "server" folder
    let secret_code = fs::read_to_string(PATH_SECRET_FILE)
        .expect("File should exist within backend, no matter what");

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

    // Add `None` fields where an empty representation has been made
    let last_name = make_option(&payload.last_name);
    let place_of_birth = make_option(&payload.place_of_birth);
    let birthday = make_date_option(&payload.birthday);

    let user = User::new(
        payload.username,
        payload.password,
        payload.first_name,
        last_name,
        birthday,
        payload.gender,
        place_of_birth
    );

    match make_user_file(&user) {
        Ok(_) => {
            // TODO: Add the user to the `AppState.users` HashMap
            let mut state = shared_state.write().unwrap();
            state.append_user(user.clone());
            let user_auth = UserAuth {
                username: user.username.to_string(),
                password: user.password.to_string()
            };
            (StatusCode::CREATED, Ok(state.generate_token(&user_auth)))
        },
        Err(message) => (
            StatusCode::BAD_REQUEST,
            Err(format!("Invalid input: {:?}", message))
        )
    }
}

/// Adds a user to the data/users folder as a separate JSON file
fn make_user_file(user: &User) -> std::io::Result<()> {
    let data = serde_json::to_string(&user);
    let file_path = format!("{}/{}.json", USR_PATH, user.username);
    fs::write(file_path, data?)?;

    // Make a folder with the user's username
    let file_path = format!("{}/{}", DIR_PATH, user.username);
    fs::create_dir(file_path)?;
    Ok(())
}

/// Transforms empty strings into None-type Options
fn make_option(payload_field: &Option<String>) -> Option<String> {
    if &payload_field.clone()?.len() == &0 {
        None
    } else {
        let opt = &payload_field.clone()?;
        Some(String::from(opt))
    }
}

/// Transforms the 0/00/0000 dates into None-type Options
fn make_date_option(date_option: &Option<Date>) -> Option<Date> {
    let curr_date = date_option.clone()
        .expect("Enforced to be valid data from front-end");
    let is_empty =
        curr_date.year() == 0
        && curr_date.month() == 0
        && curr_date.day() == 0;

    if is_empty {
        None
    } else {
        Some(curr_date)
    }
}

/// Creates a new user from the UserAuth data
pub async fn login_user(
    State(shared_state): State<Arc<RwLock<AppData>>>,
    Json(user_auth): Json<UserAuth>,
) -> impl IntoResponse {
    let mut state = shared_state.write().unwrap();
    if state.autheticate(&user_auth) {
        return (StatusCode::OK, Ok(state.generate_token(&user_auth)));
    }
    (StatusCode::NOT_FOUND, Err("Inavlid credentials"))
}

pub async fn check_valid_key(
    State(shared_state): State<Arc<RwLock<AppData>>>,
    token: String,
) -> impl IntoResponse {
    let state = shared_state.write().unwrap();
    (StatusCode::OK, Json(json!({"exists" : state.session_exists(&token)})))
}

pub async fn get_user_data(
    State(shared_state): State<Arc<RwLock<AppData>>>,
    token: String,
) -> impl IntoResponse {
    let mut state = shared_state.write().unwrap();

    let json_data;
    match state.get_user_data(&token) {
        Ok(user_data) => {
            json_data = json!({
                "exists" : true,
                "user_data" : user_data
            });
        },
        Err(reason) => {
            println!("Error: {reason}");
            json_data = json!({
                "exists" : false,
            });
        }
    };
    (StatusCode::OK, Json(json_data))
}
