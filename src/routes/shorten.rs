use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use uuid::Uuid;

use crate::structs::AppState;
use crate::structs::{ShortenRequest, ShortenResponse};

pub async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let code = Uuid::new_v4().to_string()[..6].to_string();

    let _ = sqlx::query("INSERT INTO links (code, url) VALUES (?, ?)")
        .bind(&code)
        .bind(&payload.url)
        .execute(&state.db)
        .await;

    let short_url = format!("https://raitospace.duckdns.org/api/shortener/r/{}", code);
    (StatusCode::OK, Json(ShortenResponse { short_url }))
}
