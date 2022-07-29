use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    text: String,
    user: String,
    added: DateTime<Utc>,
}

impl Message {
    pub fn new(text: String, user: String, added: DateTime<Utc>) -> Self {
        Self { text, user, added }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMessage {
    pub text: String,
    pub user: String,
}
