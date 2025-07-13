use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};

use crate::structs::{AppState, Link};


pub async fn redirect(
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