use axum::extract::{Path, State};

#[derive(Clone)]
struct AppState {
    greeting_prefix: String,
    pool: sqlx::SqlitePool,
}

#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
}

/* Basic Server Setup */
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://app.db").await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT);")
        .execute(&pool)
        .await
        .unwrap();

    let shared_state = AppState {
        greeting_prefix: String::from("Hello"),
        pool: pool,
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
    let _user = create_user(&state.pool, &name).await.unwrap();
    
    format!("{} {}!", state.greeting_prefix, name)
}

use sqlx::SqlitePool;

async fn create_user(pool: &sqlx::SqlitePool, name: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name"
    )
    .bind(name) 
    .fetch_one(pool)
    .await?;

    Ok(user)
}