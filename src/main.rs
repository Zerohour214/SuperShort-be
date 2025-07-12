use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{env, net::SocketAddr};
use uuid::Uuid;



#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
}

#[derive(FromRow)]
struct Link {
    code: String,
    url: String,
}

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
        .with_state(AppState { db: pool });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Running at http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let code = Uuid::new_v4().to_string()[..6].to_string();

    let _ = sqlx::query("INSERT INTO links (code, url) VALUES (?, ?)")
        .bind(&code)
        .bind(&payload.url)
        .execute(&state.db)
        .await;

    let short_url = format!("http://localhost:3000/r/{}", code);
    (StatusCode::OK, Json(ShortenResponse { short_url }))
}

async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Link>("SELECT code, url FROM links WHERE code = ?")
        .bind(code)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(link)) => Redirect::temporary(&link.url).into_response(),
        _ => (StatusCode::NOT_FOUND, "Link not found").into_response(),
    }
}
