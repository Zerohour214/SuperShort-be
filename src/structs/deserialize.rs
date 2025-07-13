use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}