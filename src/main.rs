mod discord;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tracing::{debug, info};

#[derive(Clone)]
struct AppState {
    discord_uid: Option<String>
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let config = AppState {
        discord_uid: std::env::var("DISCORD_UID").ok()
    };
    
    let app = Router::new()
        .route("/", get(root))
        .with_state(config);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello world"
}

async fn activity(State(state): State<AppState>) -> impl IntoResponse {
    
}