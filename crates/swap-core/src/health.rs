//! Health Integration Module
//!
//! Platform-agnostic abstraction for health data syncing.
//! iOS: HealthKit, Android: Health Connect
//!
//! HIG Requirements:
//! - Request access only when needed (not at launch)
//! - Use "the Health app" terminology in user-facing text
//! - Maintain Activity ring colors if displaying them

use crate::models::Workout;
use serde::{Deserialize, Serialize};

/// Health data permission status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum HealthPermissionStatus {
    /// Permission not yet requested
    NotDetermined,
    /// User denied access
    Denied,
    /// User granted access
    Authorized,
    /// Health data not available on this device
    Unavailable,
}

/// Types of health data we can read/write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum HealthDataType {
    /// Active energy burned (calories)
    ActiveEnergy,
    /// Workout sessions
    Workouts,
    /// Heart rate during exercise
    HeartRate,
    /// Step count
    StepCount,
    /// Weight measurements
    BodyWeight,
}

/// Workout type for health platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum HealthWorkoutType {
    TraditionalStrengthTraining,
    FunctionalStrengthTraining,
    HighIntensityIntervalTraining,
    CrossTraining,
    Other,
}

/// Data to sync to the health platform.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct HealthWorkoutData {
    /// Start time (Unix timestamp)
    pub start_time: i64,
    /// End time (Unix timestamp)
    pub end_time: i64,
    /// Type of workout
    pub workout_type: HealthWorkoutType,
    /// Calories burned
    pub calories_burned: u32,
    /// Duration in seconds
    pub duration_seconds: i64,
    /// Optional workout name/title
    pub title: Option<String>,
}

/// Activity ring data (iOS specific, but abstracted for portability).
/// Colors per HIG:
/// - Move (red): #FA114F
/// - Exercise (green): #92E82A
/// - Stand (blue): #00D4FF
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ActivityRingData {
    /// Move ring progress (0.0 - 1.0+)
    pub move_progress: f64,
    /// Move goal in calories
    pub move_goal: u32,
    /// Move current calories
    pub move_current: u32,
    /// Exercise ring progress (0.0 - 1.0+)
    pub exercise_progress: f64,
    /// Exercise goal in minutes
    pub exercise_goal: u32,
    /// Exercise current minutes
    pub exercise_current: u32,
    /// Stand ring progress (0.0 - 1.0+)
    pub stand_progress: f64,
    /// Stand goal in hours
    pub stand_goal: u32,
    /// Stand current hours
    pub stand_current: u32,
}

impl Default for ActivityRingData {
    fn default() -> Self {
        Self {
            move_progress: 0.0,
            move_goal: 500,
            move_current: 0,
            exercise_progress: 0.0,
            exercise_goal: 30,
            exercise_current: 0,
            stand_progress: 0.0,
            stand_goal: 12,
            stand_current: 0,
        }
    }
}

/// Activity ring colors as defined by Apple HIG.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ActivityRingColors {
    /// Move ring color (red)
    pub move_color: String,
    /// Exercise ring color (green)
    pub exercise_color: String,
    /// Stand ring color (blue)
    pub stand_color: String,
}

/// Get the standard Activity ring colors per HIG.
#[uniffi::export]
pub fn get_activity_ring_colors() -> ActivityRingColors {
    ActivityRingColors {
        move_color: "#FA114F".to_string(),
        exercise_color: "#92E82A".to_string(),
        stand_color: "#00D4FF".to_string(),
    }
}

/// Request structure for health permissions.
/// UI should use "the Health app" terminology as per HIG.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct HealthPermissionRequest {
    /// Types to request read access for
    pub read_types: Vec<HealthDataType>,
    /// Types to request write access for
    pub write_types: Vec<HealthDataType>,
    /// User-facing reason for the request
    pub reason: String,
}

/// Create a standard permission request for workout tracking.
#[uniffi::export]
pub fn create_workout_permission_request() -> HealthPermissionRequest {
    HealthPermissionRequest {
        read_types: vec![
            HealthDataType::ActiveEnergy,
            HealthDataType::HeartRate,
            HealthDataType::Workouts,
        ],
        write_types: vec![
            HealthDataType::ActiveEnergy,
            HealthDataType::Workouts,
        ],
        reason: "Swap syncs your workouts with the Health app to contribute to your Activity rings and track your fitness progress.".to_string(),
    }
}

/// Convert a Swap workout to health platform format.
#[uniffi::export]
pub fn workout_to_health_data(workout: &Workout) -> HealthWorkoutData {
    let end_time = workout.completed_at.unwrap_or(workout.started_at);

    HealthWorkoutData {
        start_time: workout.started_at,
        end_time,
        workout_type: HealthWorkoutType::TraditionalStrengthTraining,
        calories_burned: workout.calories_burned.unwrap_or(0),
        duration_seconds: end_time - workout.started_at,
        title: Some(workout.name.clone()),
    }
}

/// Health sync result.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct HealthSyncResult {
    pub success: bool,
    pub synced_workout_id: Option<String>,
    pub error_message: Option<String>,
    /// Activity rings after sync
    pub updated_rings: Option<ActivityRingData>,
}

/// Create a successful sync result.
#[uniffi::export]
pub fn create_sync_success(workout_id: String, rings: ActivityRingData) -> HealthSyncResult {
    HealthSyncResult {
        success: true,
        synced_workout_id: Some(workout_id),
        error_message: None,
        updated_rings: Some(rings),
    }
}

/// Create a failed sync result.
#[uniffi::export]
pub fn create_sync_failure(error: String) -> HealthSyncResult {
    HealthSyncResult {
        success: false,
        synced_workout_id: None,
        error_message: Some(error),
        updated_rings: None,
    }
}

/// Privacy policy info required by HIG.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct HealthPrivacyInfo {
    /// What data we collect
    pub data_collected: Vec<String>,
    /// How we use the data
    pub data_usage: Vec<String>,
    /// Data retention policy
    pub retention_policy: String,
    /// Whether data is shared with third parties
    pub third_party_sharing: bool,
    /// Link to full privacy policy
    pub privacy_policy_url: String,
}

/// Get the health privacy info for the app.
#[uniffi::export]
pub fn get_health_privacy_info() -> HealthPrivacyInfo {
    HealthPrivacyInfo {
        data_collected: vec![
            "Workout duration and type".to_string(),
            "Active calories burned".to_string(),
            "Exercise minutes".to_string(),
        ],
        data_usage: vec![
            "Track your workout progress".to_string(),
            "Contribute to your Activity rings".to_string(),
            "Calculate fitness metrics".to_string(),
        ],
        retention_policy: "Your health data is stored securely on your device. We do not store health data on our servers.".to_string(),
        third_party_sharing: false,
        privacy_policy_url: "https://swap.app/privacy".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_ring_colors() {
        let colors = get_activity_ring_colors();
        assert_eq!(colors.move_color, "#FA114F");
        assert_eq!(colors.exercise_color, "#92E82A");
        assert_eq!(colors.stand_color, "#00D4FF");
    }

    #[test]
    fn test_workout_to_health_data() {
        let mut workout = Workout::new("Test Workout".to_string());
        workout.calories_burned = Some(250);
        workout.completed_at = Some(workout.started_at + 3600); // 1 hour

        let health_data = workout_to_health_data(&workout);

        assert_eq!(health_data.calories_burned, 250);
        assert_eq!(health_data.duration_seconds, 3600);
        assert_eq!(health_data.title, Some("Test Workout".to_string()));
    }

    #[test]
    fn test_permission_request() {
        let request = create_workout_permission_request();

        assert!(request.write_types.contains(&HealthDataType::Workouts));
        assert!(request.reason.contains("Health app"));
    }
}
