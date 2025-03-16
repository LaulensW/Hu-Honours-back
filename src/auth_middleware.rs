use axum::{
    body::Body,
    middleware::Next,
    http::{Request, StatusCode},
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::models::Claims;

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth_header = req.headers()
        .get("Authorization")
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid header format".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization format".to_string()))?;

    let secret = "my-super-secret-key".as_bytes();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}
