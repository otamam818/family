use std::time::SystemTime;
use serde::{Deserialize, Serialize};

// In seconds
const THIRTY_MINUTES: u64 = 1800;

#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct UserSession {
    pub username: String,
    pub time: SystemTime,
}

impl UserSession {
    pub fn new(username: String) -> Self {
        Self {
            username,
            time: SystemTime::now()
        }
    }

    pub fn still_valid(&self) -> bool {
        let curr_time = SystemTime::now();
        let elapsed = curr_time
            .duration_since(self.time)
            .expect("The accessed time from self gets set during session creation")
            .as_secs();
        elapsed < THIRTY_MINUTES
    }

    pub fn revalidate(&mut self) {
        self.time = SystemTime::now()
    }
}

