mod api;
mod config;
mod process;
mod supervisor;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=String::from("supervisors.toml"))]
    configfile: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let config = config::Config::load(&args.configfile);
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
