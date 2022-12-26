use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use crate::date::Date;

/*
 * The following derive signifies a JSON output and input
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
 */

/// Represents a family member's basic info
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub birthday: Option<Date>,
    pub gender: Gender,
    pub place_of_birth: Option<String>,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        first_name: String,
        last_name: Option<String>,
        birthday: Option<Date>,
        gender: Gender,
        place_of_birth: Option<String>,
    ) -> User {
        User {
            username,
            password,
            first_name,
            last_name,
            birthday,
            gender,
            place_of_birth,
        }
    }
}

/// Represents a gender of the family
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum Gender {
    MALE,
    FEMALE,
    OTHER
}

/// Represents a family member's authetication info. Used in places like
/// during logins
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub secret_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub birthday: Option<Date>,
    pub gender: Gender,
    pub place_of_birth: Option<String>,
}

// TODO: Migrate to separate folder
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct UserSession {
    pub token: String,
    pub user: User,
    pub time: SystemTime,
}

impl UserSession {
    pub fn new(token: String, user: User) -> Self {
        Self {
            token,
            user,
            time: SystemTime::now()
        }
    }
}

