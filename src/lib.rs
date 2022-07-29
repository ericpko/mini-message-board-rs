use std::sync::{Arc, RwLock};

use models::message::Message;
use tokio::signal;

pub mod database;
mod models;
pub mod routes;

pub type Db = Arc<RwLock<Vec<Message>>>;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("\nsignal received, starting graceful shutdown");
}
