use sqlx::FromRow;

#[derive(FromRow)]
pub struct Link {
    pub url: String,
}