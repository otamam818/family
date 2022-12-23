/// The file that controls all the functions defined by the user
use std::fs;
use axum::{response::IntoResponse, Json, http::StatusCode};
use crate::{user_info::CreateUser, date::Date};

/// Creates a new user from the UserAuth data
pub async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    // The assumption is that this is run from the "server" folder
    let secret_code = fs::read_to_string("data/secret_code.txt")
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

    let user = CreateUser::new(
        payload.username,
        payload.password,
        payload.secret_code,
        payload.first_name,
        last_name,
        birthday,
        payload.gender,
        place_of_birth
    );

    match make_user_file(&user) {
        Ok(_) => (
            StatusCode::CREATED,
            Ok(Json(user))
        ),
        Err(message) => (
            StatusCode::BAD_REQUEST,
            Err(format!("Invalid input: {:?}", message))
        )
    }
}

fn make_user_file(user: &CreateUser) -> std::io::Result<()> {
    let data = serde_json::to_string(&user);
    let file_path = format!("data/users/{}.json", user.username);
    fs::write(file_path, data.unwrap())?;
    Ok(())
}

fn make_option(payload_field: &Option<String>) -> Option<String> {
    if &payload_field.clone().unwrap().len() == &0 {
        None
    } else {
        let opt = &payload_field.clone().unwrap();
        Some(String::from(opt))
    }
}

fn make_date_option(date_option: &Option<Date>) -> Option<Date> {
    let curr_date = date_option.clone().unwrap();
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

