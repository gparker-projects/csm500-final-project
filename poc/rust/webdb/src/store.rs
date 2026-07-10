use sqlx::postgres::{PgPoolOptions, PgPool}; //, PgRow};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        Ok()
    }
}
