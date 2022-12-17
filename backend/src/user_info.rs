use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}


