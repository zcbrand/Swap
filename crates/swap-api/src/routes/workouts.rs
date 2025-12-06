//! Workout API endpoints.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::{ApiError, Result};
use crate::state::AppState;

/// Build the workouts router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_workouts).post(create_workout))
        .route("/{id}", get(get_workout).delete(delete_workout))
        .route("/{id}/complete", post(complete_workout))
}

#[derive(Debug, Deserialize)]
pub struct ListWorkoutsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkoutListResponse {
    pub workouts: Vec<WorkoutRecord>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WorkoutRecord {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub synced_to_health: bool,
}

/// List workouts with pagination.
pub async fn list_workouts(
    State(state): State<AppState>,
    Query(query): Query<ListWorkoutsQuery>,
) -> Result<Json<WorkoutListResponse>> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let workouts: Vec<WorkoutRecord> = sqlx::query_as(
        r#"
        SELECT id, user_id, name, started_at, completed_at, synced_to_health
        FROM workouts
        ORDER BY started_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM workouts")
        .fetch_one(&state.db)
        .await?;

    Ok(Json(WorkoutListResponse {
        workouts,
        total: total.0,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkoutRequest {
    pub user_id: String,
    pub name: String,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct CreateWorkoutResponse {
    pub id: String,
    pub created_at: i64,
}

/// Create a new workout.
pub async fn create_workout(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkoutRequest>,
) -> Result<Json<CreateWorkoutResponse>> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO workouts (id, user_id, name, data, started_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.user_id)
    .bind(&req.name)
    .bind(&req.data)
    .bind(now)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;

    Ok(Json(CreateWorkoutResponse {
        id,
        created_at: now,
    }))
}

#[derive(Debug, Serialize, FromRow)]
pub struct WorkoutDetailResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub data: String,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub synced_to_health: bool,
}

/// Get a single workout by ID.
pub async fn get_workout(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<WorkoutDetailResponse>> {
    let workout: Option<WorkoutDetailResponse> = sqlx::query_as(
        r#"
        SELECT id, user_id, name, data, started_at, completed_at, synced_to_health
        FROM workouts
        WHERE id = ?
        "#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;

    let workout = workout.ok_or_else(|| ApiError::NotFound(format!("Workout {} not found", id)))?;

    Ok(Json(workout))
}

/// Delete a workout.
pub async fn delete_workout(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM workouts WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Workout {} not found", id)));
    }

    Ok(Json(serde_json::json!({ "deleted": true })))
}

#[derive(Debug, Deserialize)]
pub struct CompleteWorkoutRequest {
    pub data: String,
    pub calories_burned: Option<u32>,
}

/// Mark a workout as completed.
pub async fn complete_workout(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<CompleteWorkoutRequest>,
) -> Result<Json<serde_json::Value>> {
    let now = Utc::now().timestamp();

    let result = sqlx::query(
        r#"
        UPDATE workouts
        SET completed_at = ?, data = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(now)
    .bind(&req.data)
    .bind(now)
    .bind(&id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Workout {} not found", id)));
    }

    Ok(Json(serde_json::json!({
        "completed": true,
        "completed_at": now
    })))
}
