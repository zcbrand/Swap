//! Data synchronization endpoints.
//!
//! Handles sync between mobile clients and the server.
//! Critical for preventing data loss (major user frustration point).

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::state::AppState;

/// Build the sync router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(sync_status))
        .route("/push", post(push_changes))
        .route("/pull", get(pull_changes))
}

#[derive(Debug, Serialize)]
pub struct SyncStatusResponse {
    pub last_sync: Option<i64>,
    pub pending_changes: u32,
    pub server_version: String,
}

/// Get sync status for a user.
pub async fn sync_status(
    State(_state): State<AppState>,
) -> Result<Json<SyncStatusResponse>> {
    // In a full implementation, this would check the user's sync state
    Ok(Json(SyncStatusResponse {
        last_sync: Some(Utc::now().timestamp()),
        pending_changes: 0,
        server_version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct PushChangesRequest {
    pub user_id: String,
    pub device_id: String,
    pub changes: Vec<SyncChange>,
    pub last_sync_timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncChange {
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String,
    pub data: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct PushChangesResponse {
    pub accepted: u32,
    pub conflicts: Vec<SyncConflict>,
    pub server_timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct SyncConflict {
    pub entity_type: String,
    pub entity_id: String,
    pub client_timestamp: i64,
    pub server_timestamp: i64,
    pub resolution: String,
}

/// Push local changes to server.
pub async fn push_changes(
    State(_state): State<AppState>,
    Json(req): Json<PushChangesRequest>,
) -> Result<Json<PushChangesResponse>> {
    // In a full implementation, this would:
    // 1. Validate the changes
    // 2. Check for conflicts with server data
    // 3. Apply non-conflicting changes
    // 4. Return conflicts for client resolution

    let accepted = req.changes.len() as u32;

    Ok(Json(PushChangesResponse {
        accepted,
        conflicts: vec![],
        server_timestamp: Utc::now().timestamp(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct PullChangesQuery {
    pub user_id: String,
    pub since: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PullChangesResponse {
    pub changes: Vec<SyncChange>,
    pub has_more: bool,
    pub server_timestamp: i64,
}

/// Pull changes from server since last sync.
pub async fn pull_changes(
    State(_state): State<AppState>,
    Query(_query): Query<PullChangesQuery>,
) -> Result<Json<PullChangesResponse>> {
    // In a full implementation, this would:
    // 1. Fetch all changes since the given timestamp
    // 2. Filter by user
    // 3. Return paginated results

    Ok(Json(PullChangesResponse {
        changes: vec![],
        has_more: false,
        server_timestamp: Utc::now().timestamp(),
    }))
}
