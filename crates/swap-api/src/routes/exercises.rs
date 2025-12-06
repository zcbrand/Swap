//! Exercise API endpoints.

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::{ApiError, Result};
use crate::state::AppState;

/// Build the exercises router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_exercises).post(create_exercise))
        .route("/{id}", get(get_exercise))
        .route("/search", get(search_exercises))
}

#[derive(Debug, Deserialize)]
pub struct ListExercisesQuery {
    pub muscle_group: Option<String>,
    pub exercise_type: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ExerciseRecord {
    pub id: String,
    pub name: String,
    pub exercise_type: String,
    pub primary_muscle: String,
    pub secondary_muscles: Option<String>,
    pub instructions: Option<String>,
    pub is_custom: bool,
}

/// List exercises with optional filtering.
pub async fn list_exercises(
    State(state): State<AppState>,
    Query(query): Query<ListExercisesQuery>,
) -> Result<Json<Vec<ExerciseRecord>>> {
    let limit = query.limit.unwrap_or(100).min(500);

    let exercises: Vec<ExerciseRecord> = if let Some(muscle) = query.muscle_group {
        sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, primary_muscle, secondary_muscles,
                   instructions, is_custom
            FROM exercises
            WHERE primary_muscle = ?
            ORDER BY name
            LIMIT ?
            "#,
        )
        .bind(&muscle)
        .bind(limit)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, primary_muscle, secondary_muscles,
                   instructions, is_custom
            FROM exercises
            ORDER BY name
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(exercises))
}

#[derive(Debug, Deserialize)]
pub struct CreateExerciseRequest {
    pub name: String,
    pub exercise_type: String,
    pub primary_muscle: String,
    pub secondary_muscles: Option<Vec<String>>,
    pub instructions: Option<String>,
}

/// Create a custom exercise.
pub async fn create_exercise(
    State(state): State<AppState>,
    Json(req): Json<CreateExerciseRequest>,
) -> Result<Json<ExerciseRecord>> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    let secondary_muscles = req.secondary_muscles.map(|m| m.join(","));

    sqlx::query(
        r#"
        INSERT INTO exercises (id, name, exercise_type, primary_muscle, secondary_muscles, instructions, is_custom, created_at)
        VALUES (?, ?, ?, ?, ?, ?, 1, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.exercise_type)
    .bind(&req.primary_muscle)
    .bind(&secondary_muscles)
    .bind(&req.instructions)
    .bind(now)
    .execute(&state.db)
    .await?;

    Ok(Json(ExerciseRecord {
        id,
        name: req.name,
        exercise_type: req.exercise_type,
        primary_muscle: req.primary_muscle,
        secondary_muscles,
        instructions: req.instructions,
        is_custom: true,
    }))
}

/// Get a single exercise by ID.
pub async fn get_exercise(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ExerciseRecord>> {
    let exercise: Option<ExerciseRecord> = sqlx::query_as(
        r#"
        SELECT id, name, exercise_type, primary_muscle, secondary_muscles,
               instructions, is_custom
        FROM exercises
        WHERE id = ?
        "#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;

    let exercise =
        exercise.ok_or_else(|| ApiError::NotFound(format!("Exercise {} not found", id)))?;

    Ok(Json(exercise))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<i64>,
}

/// Search exercises by name.
pub async fn search_exercises(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<ExerciseRecord>>> {
    let limit = query.limit.unwrap_or(20).min(100);
    let search_term = format!("%{}%", query.q);

    let exercises: Vec<ExerciseRecord> = sqlx::query_as(
        r#"
        SELECT id, name, exercise_type, primary_muscle, secondary_muscles,
               instructions, is_custom
        FROM exercises
        WHERE name LIKE ?
        ORDER BY name
        LIMIT ?
        "#,
    )
    .bind(&search_term)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(exercises))
}
