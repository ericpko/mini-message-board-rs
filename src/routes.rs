use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;

use crate::{
    models::message::{CreateMessage, Message},
    Db,
};

pub async fn index(Extension(db): Extension<Db>) -> impl IntoResponse {
    let messages = db.read().unwrap();
    let messages = messages.to_vec();

    Json(messages)
}

pub async fn create_message(
    Json(input): Json<CreateMessage>,
    db: Extension<Db>,
) -> impl IntoResponse {
    let db = db.0; // not needed if using pattern matching syntax shown in <index>

    let message = Message::new(input.text, input.user, Utc::now());
    db.write().unwrap().push(message.clone());

    (StatusCode::CREATED, Json(message))
    // Redirect::to("/")
}
