use std::{collections::HashMap, fs};
use std::hash::{Hash, Hasher};
use std::io::Result;

use serde::{Deserialize, Serialize};
use crate::user_info::{User, UserAuth};

// Enforce front-end
const ENF_FE: &'static str
    = "Well-formed contents should be enforced from the front-end";

/// Stores the state of different pre-loaded data
#[derive(Debug, Serialize,Deserialize, Clone, Eq, PartialEq)]
pub struct AppData {
    users: HashMap<String, User>
}

impl Hash for AppData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.users.len().hash(state);
    }
}

// Assumed to run from the root server folder
const USER_DIR: &'static str = "./data/users";

impl AppData {
    pub fn load() -> AppData {
        let mut users: HashMap<String, User> = HashMap::new();
        // Get a list of files in data/users/
        let user_filepaths = match get_filepaths(USER_DIR) {
            Ok(paths) => paths,
            Err(msg) => panic!("Invalid filepath: {}", msg)
        };

        // Load each of the files as a User to add to the final map
        for path in user_filepaths {
            let contents = fs::read_to_string(path).unwrap();
            let user: User = serde_json::from_str(&contents).expect(ENF_FE);
            users.insert(user.username.clone(), user);
        }

        AppData {
            users
        }
    }

    /// Checks if the provided credentials matches with the pre-loaded data
    /// or not.
    pub fn autheticate(&self, user_auth: &UserAuth) -> bool{
        if self.user_exists(&user_auth.username) {
            return self.password_matches(user_auth);
        }
        false
    }

    fn user_exists(&self, username: &String) -> bool {
        self.users.contains_key(username)
    }

    fn password_matches(&self, user_auth: &UserAuth) -> bool {
        self.users.get(&user_auth.username).unwrap()
            .password
            .eq(&user_auth.password)
    }
}

/// Gets the children files and directories found in the string
fn get_filepaths(path: &str) -> Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .map(|f|
            f.expect("It exists, so it can be unwrapped")
             .path()
             .to_str()
             .unwrap()
             .to_string()
        ).collect::<Vec<String>>())
}

