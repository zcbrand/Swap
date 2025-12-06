//! Widget Data Providers
//!
//! Provides data for iOS WidgetKit and Android Glance widgets.
//! HIG Requirements:
//! - Widgets should be glanceable
//! - Display most significant metrics without requiring app launch
//! - Support small, medium, and large sizes with appropriate information density

use crate::health::ActivityRingData;
use crate::metrics::{WeeklyGoals, WorkoutStreak};
use crate::models::Workout;
use serde::{Deserialize, Serialize};

/// Widget size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum WidgetSize {
    /// Small: Single metric focus
    Small,
    /// Medium: 2-3 metrics
    Medium,
    /// Large: Full summary
    Large,
    /// Accessory (watchOS/Lock Screen)
    Accessory,
}

/// Widget family types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum WidgetFamily {
    /// Quick start workout widget
    QuickStart,
    /// Progress and streak widget
    Progress,
    /// Activity rings widget
    ActivityRings,
    /// Next scheduled workout
    NextWorkout,
    /// Recent workout summary
    RecentWorkout,
}

/// Data for the Quick Start widget.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct QuickStartWidgetData {
    /// Recent/favorite workout templates
    pub quick_workouts: Vec<QuickWorkoutEntry>,
    /// Whether there's an active workout
    pub has_active_workout: bool,
    /// Active workout name if any
    pub active_workout_name: Option<String>,
}

/// Entry for quick start workout list.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct QuickWorkoutEntry {
    pub id: String,
    pub name: String,
    pub exercise_count: u32,
    pub estimated_minutes: u32,
    /// Deep link URL to start this workout
    pub deep_link: String,
}

/// Data for the Progress widget.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ProgressWidgetData {
    /// Current workout streak
    pub streak: WorkoutStreak,
    /// Weekly goals progress
    pub weekly_goals: WeeklyGoals,
    /// Workouts this week
    pub workouts_this_week: u32,
    /// Total calories this week
    pub calories_this_week: u32,
    /// Total volume this week (kg)
    pub volume_this_week: f64,
}

/// Data for the Activity Rings widget.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ActivityRingsWidgetData {
    /// Current activity ring data
    pub rings: ActivityRingData,
    /// Whether we have permission to show rings
    pub has_health_permission: bool,
    /// Last workout contribution
    pub last_workout_contribution: Option<WorkoutContribution>,
}

/// Contribution from last workout to rings.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutContribution {
    pub workout_name: String,
    pub calories_added: u32,
    pub minutes_added: u32,
}

/// Data for the Recent Workout widget.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct RecentWorkoutWidgetData {
    /// Last workout summary
    pub last_workout: Option<WorkoutSummaryWidget>,
    /// Days since last workout
    pub days_ago: u32,
}

/// Compact workout summary for widgets.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutSummaryWidget {
    pub name: String,
    pub duration_minutes: u32,
    pub sets_completed: u32,
    pub volume_kg: f64,
    pub calories: u32,
    pub timestamp: i64,
}

/// Data for the Next Workout widget.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct NextWorkoutWidgetData {
    /// Next scheduled workout (if any)
    pub next_workout: Option<ScheduledWorkout>,
    /// Suggested workout based on history
    pub suggested_workout: Option<QuickWorkoutEntry>,
    /// Muscle groups that haven't been trained recently
    pub recovery_info: Vec<MuscleRecoveryInfo>,
}

/// Scheduled workout info.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ScheduledWorkout {
    pub id: String,
    pub name: String,
    pub scheduled_time: i64,
    pub exercise_count: u32,
}

/// Muscle recovery information.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct MuscleRecoveryInfo {
    pub muscle_group: String,
    pub days_since_trained: u32,
    pub is_recovered: bool,
}

/// Create widget data for a specific size.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WidgetContent {
    pub size: WidgetSize,
    pub family: WidgetFamily,
    /// Primary metric to display
    pub primary_value: String,
    /// Primary metric label
    pub primary_label: String,
    /// Secondary metrics (for medium/large)
    pub secondary_metrics: Vec<WidgetMetric>,
    /// Deep link URL
    pub deep_link: String,
    /// Last update timestamp
    pub last_updated: i64,
}

/// A single metric for widget display.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WidgetMetric {
    pub label: String,
    pub value: String,
    pub icon: String,
}

/// Create progress widget data.
#[uniffi::export]
pub fn create_progress_widget(
    streak: WorkoutStreak,
    goals: WeeklyGoals,
    workouts_this_week: u32,
    calories_this_week: u32,
    volume_this_week: f64,
) -> ProgressWidgetData {
    ProgressWidgetData {
        streak,
        weekly_goals: goals,
        workouts_this_week,
        calories_this_week,
        volume_this_week,
    }
}

/// Create quick start widget data.
#[uniffi::export]
pub fn create_quick_start_widget(
    quick_workouts: Vec<QuickWorkoutEntry>,
    active_workout: Option<String>,
) -> QuickStartWidgetData {
    QuickStartWidgetData {
        quick_workouts,
        has_active_workout: active_workout.is_some(),
        active_workout_name: active_workout,
    }
}

/// Convert a workout to widget summary format.
#[uniffi::export]
pub fn workout_to_widget_summary(workout: &Workout) -> WorkoutSummaryWidget {
    WorkoutSummaryWidget {
        name: workout.name.clone(),
        duration_minutes: (workout.duration_seconds() / 60) as u32,
        sets_completed: workout.total_sets(),
        volume_kg: workout.total_volume_kg(),
        calories: workout.calories_burned.unwrap_or(0),
        timestamp: workout.completed_at.unwrap_or(workout.started_at),
    }
}

/// Get appropriate content for widget size.
#[uniffi::export]
pub fn get_widget_content(
    size: WidgetSize,
    family: WidgetFamily,
    progress: &ProgressWidgetData,
) -> WidgetContent {
    let (primary_value, primary_label, secondary_metrics) = match (size, family) {
        (WidgetSize::Small, WidgetFamily::Progress) => (
            format!("{}", progress.streak.current_streak),
            "Day Streak".to_string(),
            vec![],
        ),
        (WidgetSize::Medium, WidgetFamily::Progress) => (
            format!("{}", progress.streak.current_streak),
            "Day Streak".to_string(),
            vec![
                WidgetMetric {
                    label: "This Week".to_string(),
                    value: format!("{} workouts", progress.workouts_this_week),
                    icon: "dumbbell".to_string(),
                },
                WidgetMetric {
                    label: "Calories".to_string(),
                    value: format!("{}", progress.calories_this_week),
                    icon: "flame".to_string(),
                },
            ],
        ),
        (WidgetSize::Large, WidgetFamily::Progress) => (
            format!("{}", progress.streak.current_streak),
            "Day Streak".to_string(),
            vec![
                WidgetMetric {
                    label: "Workouts".to_string(),
                    value: format!(
                        "{}/{}",
                        progress.weekly_goals.completed_workouts,
                        progress.weekly_goals.target_workouts
                    ),
                    icon: "dumbbell".to_string(),
                },
                WidgetMetric {
                    label: "Minutes".to_string(),
                    value: format!(
                        "{}/{}",
                        progress.weekly_goals.completed_minutes,
                        progress.weekly_goals.target_minutes
                    ),
                    icon: "clock".to_string(),
                },
                WidgetMetric {
                    label: "Calories".to_string(),
                    value: format!(
                        "{}/{}",
                        progress.weekly_goals.burned_calories,
                        progress.weekly_goals.target_calories
                    ),
                    icon: "flame".to_string(),
                },
                WidgetMetric {
                    label: "Volume".to_string(),
                    value: format!("{:.0} kg", progress.volume_this_week),
                    icon: "scalemass".to_string(),
                },
            ],
        ),
        (WidgetSize::Accessory, _) => (
            format!("{}", progress.streak.current_streak),
            "days".to_string(),
            vec![],
        ),
        _ => (
            format!("{}", progress.workouts_this_week),
            "Workouts".to_string(),
            vec![],
        ),
    };

    WidgetContent {
        size,
        family,
        primary_value,
        primary_label,
        secondary_metrics,
        deep_link: "swap://widget/progress".to_string(),
        last_updated: chrono::Utc::now().timestamp(),
    }
}

/// Timeline entry for widget updates.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WidgetTimelineEntry {
    pub date: i64,
    pub content: WidgetContent,
}

/// Get widget refresh policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum WidgetRefreshPolicy {
    /// Refresh after workout completes
    AfterWorkout,
    /// Refresh at start of each day
    Daily,
    /// Refresh every hour
    Hourly,
    /// Never auto-refresh
    Never,
}

/// Get recommended refresh policy for widget family.
#[uniffi::export]
pub fn get_refresh_policy(family: WidgetFamily) -> WidgetRefreshPolicy {
    match family {
        WidgetFamily::QuickStart => WidgetRefreshPolicy::AfterWorkout,
        WidgetFamily::Progress => WidgetRefreshPolicy::Daily,
        WidgetFamily::ActivityRings => WidgetRefreshPolicy::Hourly,
        WidgetFamily::NextWorkout => WidgetRefreshPolicy::Hourly,
        WidgetFamily::RecentWorkout => WidgetRefreshPolicy::AfterWorkout,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_content_small() {
        let progress = ProgressWidgetData {
            streak: WorkoutStreak {
                current_streak: 5,
                longest_streak: 10,
                worked_out_today: true,
                days_since_last_workout: 0,
            },
            weekly_goals: WeeklyGoals::default(),
            workouts_this_week: 3,
            calories_this_week: 750,
            volume_this_week: 2500.0,
        };

        let content = get_widget_content(WidgetSize::Small, WidgetFamily::Progress, &progress);

        assert_eq!(content.primary_value, "5");
        assert_eq!(content.primary_label, "Day Streak");
        assert!(content.secondary_metrics.is_empty());
    }

    #[test]
    fn test_widget_content_large() {
        let progress = ProgressWidgetData {
            streak: WorkoutStreak::default(),
            weekly_goals: WeeklyGoals::default(),
            workouts_this_week: 2,
            calories_this_week: 500,
            volume_this_week: 1500.0,
        };

        let content = get_widget_content(WidgetSize::Large, WidgetFamily::Progress, &progress);

        assert_eq!(content.secondary_metrics.len(), 4);
    }

    #[test]
    fn test_refresh_policy() {
        assert_eq!(
            get_refresh_policy(WidgetFamily::Progress),
            WidgetRefreshPolicy::Daily
        );
        assert_eq!(
            get_refresh_policy(WidgetFamily::ActivityRings),
            WidgetRefreshPolicy::Hourly
        );
    }
}
