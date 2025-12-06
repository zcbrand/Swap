//! Error types for the Swap core library.

use thiserror::Error;

/// Core error type for all Swap operations.
#[derive(Debug, Error, uniffi::Error)]
pub enum SwapError {
    #[error("Invalid workout data: {message}")]
    InvalidWorkout { message: String },

    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: String },

    #[error("Exercise not found: {exercise_id}")]
    ExerciseNotFound { exercise_id: String },

    #[error("Invalid rep count for 1RM calculation: {reps} (must be 1-30)")]
    InvalidRepCount { reps: u32 },

    #[error("Health data access denied")]
    HealthAccessDenied,

    #[error("Health data sync failed: {reason}")]
    HealthSyncFailed { reason: String },

    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },
}

impl From<serde_json::Error> for SwapError {
    fn from(e: serde_json::Error) -> Self {
        SwapError::SerializationError {
            message: e.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, SwapError>;
