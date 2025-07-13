use serde::Serialize;

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}