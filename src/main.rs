mod api;
mod config;
mod process;
mod supervisor;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::load("supervisors.toml");
    let state = supervisor::run_supervisor(&config).await;

    let port = config.listen.unwrap_or("localhost:3000".to_string());

    if !port.is_empty() {
        api::listen(port, state.clone()).await
    }

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for SIGINT");
    println!("SIGINT received. Exiting...");

    supervisor::shutdown_all(state).await;
}
