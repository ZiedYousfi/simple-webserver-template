use axum::{routing::get, Form, Router, Json};
use std::net::SocketAddr;

fn check_env_vars() {
    dotenvy::dotenv().ok();
    let required_vars = ["RUST_LOG", "RUST_BACKTRACE"];

    let mut missing_or_empty_vars = Vec::new();

    for var in required_vars {
        match std::env::var(var) {
            Ok(val) if !val.trim().is_empty() => {}
            _ => {
                log::error!("Missing or empty required environment variable: {var}");
                missing_or_empty_vars.push(var);
            }
        }
    }

    if !missing_or_empty_vars.is_empty() {
        log::error!(
            "Exiting due to missing or empty environment variables: {missing_or_empty_vars:?}"
        );
        std::process::exit(1);
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = env_logger::try_init() {
        eprintln!("Failed to initialize logger: {e}");
        std::process::exit(1);
    }
    check_env_vars();
    let app = Router::new().nest("/api", Router::new().route("/", get(root).post(root_post)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    log::info!("Listening on http://{addr}");

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        log::error!("Server error: {e}");
        std::process::exit(1);
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn root_post(Form(payload): Form<std::collections::HashMap<String, String>>) -> Json<String> {
    Json(format!("Received POST request with payload: {payload:?}"))
}
