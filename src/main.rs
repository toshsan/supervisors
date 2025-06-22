use axum::{Json, Router, extract::State, routing::get};

mod config;
mod process;
mod supervisor;

use crate::supervisor::{ProcessStatus, SupervisorState, run_supervisor};

async fn health() -> &'static str {
    "Supervisor Running. Check /status"
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

    if !port.is_empty() {
        let app = Router::new()
            .route("/", get(health))
            .route("/status", get(status))
            .with_state(state.clone());

        println!("API listening on http://{port}");
        let listener = tokio::net::TcpListener::bind(port).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for SIGINT");
    println!("SIGINT received. Exiting...");

    supervisor::shutdown_all(state).await;
}
