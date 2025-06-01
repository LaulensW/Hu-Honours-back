use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx;

use crate::models::{AuthResponse, Claims, LoginRequest, RegisterRequest, User};
use crate::AppState;

pub async fn root() -> &'static str {
    "Root test endpoint"
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username already taken".to_string(),
        ));
    }

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(&payload.username)
        .bind(&password_hash)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        message: "Registration successful".to_string(),
        token: None,
    }))
}
//sql queries apart from the main function
//.example toevoegen
//unit/integratgie tests
//user extraction function
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        &payload.username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if matches!(&user, Some(user) if verify(&payload.password, &user.password_hash)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?)
    {
        let user = user.unwrap();

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            username: user.username.clone(),
            exp: expiration,
        };

        let secret = "my-super-secret-key".as_bytes();
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(AuthResponse {
            message: "Login successful".to_string(),
            token: Some(token),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
    }
    }

pub async fn profile(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    Ok(Json(AuthResponse {
        message: format!("Welcome back, {}!", user.username),
        token: None,
    }))
}

pub async fn dashboard(
    Extension(claims): Extension<Claims>,
    State(_state): State<AppState>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    Ok(Json(AuthResponse {
        message: format!(
            "Dashboard for user {} (ID: {})",
            claims.username, claims.sub
        ),
        token: None,
    }))
}
