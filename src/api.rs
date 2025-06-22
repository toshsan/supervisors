use axum::{extract::State, routing::get, serve, Json, Router};

use crate::supervisor::{ProcessStatus, SupervisorState};

async fn health() -> &'static str {
    "Supervisor Running. Check /status"
}

async fn status(State(state): State<SupervisorState>) -> Json<Vec<ProcessStatus>> {
    let statuses = state.statuses.lock().await;
    Json(statuses.values().cloned().collect())
}

pub async fn listen(port: String, state: SupervisorState) {
    let app = Router::new()
        .route("/", get(health))
        .route("/status", get(status))
        .with_state(state);

    println!("API listening on http://{port}");
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    serve(listener, app).await.unwrap();
}
