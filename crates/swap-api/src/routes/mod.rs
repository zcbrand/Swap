//! API route handlers.

mod health;
mod workouts;
mod exercises;
mod sync;

use axum::{routing::get, Router};
use crate::state::AppState;

/// Build the API router.
pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/workouts", workouts::router())
        .nest("/exercises", exercises::router())
        .nest("/sync", sync::router())
}
