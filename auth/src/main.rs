mod dto;
mod models;

use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{io, sync::Arc};

use crate::{
    dto::{AuthResponse, LoginRequest, RegisterReqeust},
    models::{AppError, User},
};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("missing db url");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    println!("Connected to PostreSQL");

    let app_state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        // .route("/servers", get(get_server_list))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&req.username)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

    let valid = verify(&req.password, &user.password_hash)
        .map_err(|_| AppError::Internal("Failed to verify password".into()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    Ok(Json(AuthResponse { id: user.id }))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterReqeust>,
) -> Result<Json<AuthResponse>, AppError> {
    if req.username.len() < 3 || req.password.len() < 3 {
        return Err(AppError::BadRequest(
            "Username must be at least 3 characters and password at least 3".into(),
        ));
    }

    let password_hash = hash(req.password, DEFAULT_COST)
        .map_err(|_| AppError::Internal("Failed to hash password".into()))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *",
    )
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") {
            AppError::Conflict("Username already exists".into())
        } else {
            AppError::Internal(e.to_string())
        }
    })?;

    Ok(Json(AuthResponse { id: user.id }))
}
