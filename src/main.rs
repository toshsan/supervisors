mod config;
mod process;
mod supervisor;

use axum::extract::State;
use axum::{Json, Router, routing::get};
use tracing_subscriber;

use crate::supervisor::{ProcessStatus, SupervisorState, run_supervisor};

async fn health() -> &'static str {
    "Supervisor OK"
}

async fn status(State(state): State<SupervisorState>) -> Json<Vec<ProcessStatus>> {
    let statuses = state.statuses.lock().await;
    Json(statuses.values().cloned().collect())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::load("supervisors.toml");
    let state = run_supervisor(&config).await;

    let port = config.listen.unwrap_or("localhost:3000".to_string());
    if port.is_empty() {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for SIGINT");
        println!("SIGINT received. Exiting...");
        return;
    }
    // Serve API
    let app = Router::new()
        .route("/health", get(health))
        .route("/status", get(status))
        .with_state(state);

    println!("API running on {port}");
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
