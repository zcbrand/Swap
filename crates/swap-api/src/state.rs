//! Application state shared across handlers.

use sqlx::SqlitePool;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}
