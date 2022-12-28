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

/// Represents a family member's non-sensitive info
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct UserBasicData {
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub birthday: Option<Date>,
    pub gender: Gender,
    pub place_of_birth: Option<String>,
}

impl UserBasicData {
    #[allow(non_snake_case)]
    pub fn from_User (user: &User) -> Self {
        let clone = user.clone();
        Self {
            username: clone.username,
            first_name: clone.first_name,
            last_name: clone.last_name,
            birthday: clone.birthday,
            gender: clone.gender,
            place_of_birth: clone.place_of_birth
        }
    }
}

