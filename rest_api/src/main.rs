use axum::extract::{Path, State};

#[derive(Clone)]
struct AppState {
    greeting_prefix: String,
    pool: sqlx::SqlitePool,
}

use serde::Serialize;

#[derive(Serialize, FromRow)]
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

use axum::http::StatusCode;

async fn greet(Path(name): Path<String>, State(state): State<AppState>) -> Result<String, (StatusCode, String)> {

    if name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Error: Name connot be empty!".to_string()));
    }

    let _user = create_user(&state.pool, &name).await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save user to the database!".to_string())
    })?;

    Ok(format!("{} {}!", state.greeting_prefix, name.trim()))
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

async fn get_all_users(pool: &sqlx::SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users;")
    .fetch_all(pool)
    .await?;

    Ok(users)
}

use axum::Json;
use sqlx::prelude::FromRow;

async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = get_all_users(&state.pool).await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users".to_string())
    })?;

    Ok(Json(users))
}