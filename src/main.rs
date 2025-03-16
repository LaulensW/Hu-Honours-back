use axum::{Router, middleware};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio;

mod handlers;
mod auth_middleware;
mod models;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:Solexjedragon12%2C%2C%3F%3F@localhost:5432/honours";
    println!("Connecting to: {}", database_url);
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let state = AppState { db };

    // Public routes (no middleware)
    let public_routes = Router::new()
        .route("/", axum::routing::get(handlers::root))
        .route("/register", axum::routing::post(handlers::register))
        .route("/login", axum::routing::post(handlers::login));

    // Protected routes (with middleware)
    let protected_routes = Router::new()
        .route("/profile", axum::routing::get(handlers::profile))
        .route("/dashboard", axum::routing::get(handlers::dashboard))
        .layer(middleware::from_fn(auth_middleware::auth_middleware));

    // Combine routers
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}