use chrono::Utc;

use crate::{models::message::Message, Db};

pub fn init_database(db: &mut Db) {
    let messages = vec![
        Message::new("Hi there!".to_string(), "Amando".to_string(), Utc::now()),
        Message::new(
            "Hello World!".to_string(),
            "Charles".to_string(),
            Utc::now(),
        ),
    ];

    db.write().unwrap().extend(messages);
}
