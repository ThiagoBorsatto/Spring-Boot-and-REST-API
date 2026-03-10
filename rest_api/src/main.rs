use axum::extract::{Path, State};

#[derive(Clone)]
struct AppState {
    greeting_prefix: String,
    pool: sqlx::SqlitePool,
}

use axum::routing::{delete, put};
use serde::Serialize;

#[derive(Serialize, FromRow)]
struct User {
    id: i64,
    name: String,
}

use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateUserRequest {
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
        .route("/users", get(get_users))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
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
use tokio::runtime::Id;

async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = get_all_users(&state.pool).await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users".to_string())
    })?;

    Ok(Json(users))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>
) -> Result<String, (StatusCode, String)> {
    
    if payload.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name cannot be empty!".to_string()));
    }

    let result = sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind(payload.name.trim())
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    Ok(format!("User {} update successfully!", id))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<String, (StatusCode, String)> {
    
    let result = sqlx::query("DELETE * FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;

    
    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "User not found!".to_string()));
    }

    Ok(format!("User {} deleted successfully!", id))
}