use serde::{Deserialize, Serialize};

/*
 * The following derive signifies a JSON input
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]

 * The following derive signifies a JSON output
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
 */

/// Represents a family member's basic info
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
}

/// Represents a family member's authetication info
#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

