use std::{collections::HashMap, fs};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use crate::user_info::User;

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
        let user_filepaths = get_filepaths(USER_DIR);

        // TODO: Load each of the files as a User to add to a final map
        for path in user_filepaths {
            let contents = fs::read_to_string(path).unwrap();
            let user: User = serde_json::from_str(&contents).unwrap();
            users.insert(user.username.clone(), user);
        }

        AppData {
            users
        }
    }

    pub fn user_exists(&self, username: &String) -> bool {
        self.users.contains_key(username)
    }
}

fn get_filepaths(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|f|
            f.unwrap()
             .path()
             .to_str()
             .unwrap()
             .to_string()
        ).collect::<Vec<String>>()
}

