mod routes;
mod structs;

use axum::{
    routing::{get, post},
    Router,
};

use dotenvy::dotenv;
use sqlx::{sqlite::SqlitePoolOptions};
use std::{env, net::SocketAddr};

use crate::routes::redirect::redirect;
use crate::routes::shorten::shorten;
use crate::structs::AppState;

use tower_http::cors::{CorsLayer, Any};
use http::Method;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS links (
            code TEXT PRIMARY KEY,
            url TEXT NOT NULL
        );",
    )
        .execute(&pool)
        .await?;

    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/r/{code}", get(redirect))
        .with_state(AppState { db: pool })
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Running at http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
