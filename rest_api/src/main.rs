use axum::extract::{Path, State};

#[derive(Clone)]
struct AppState {
    greeting_prefix: String,
}

/* Basic Server Setup */
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        greeting_prefix: String::from("Hello"),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello from Rust"}))
        .route("/greet/:name", get(greet))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Rust server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn greet(Path(name): Path<String>, State(state): State<AppState>) -> String {
    format!("{} {}!", state.greeting_prefix, name)
}