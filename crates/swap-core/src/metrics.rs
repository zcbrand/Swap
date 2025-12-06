//! Metrics and analytics for workout tracking.
//!
//! Tracks user progress, workout streaks, and provides data for widgets.

use crate::models::Workout;
use serde::{Deserialize, Serialize};

/// Aggregated metrics for a time period.
#[derive(Debug, Clone, Default, Serialize, Deserialize, uniffi::Record)]
pub struct PeriodMetrics {
    /// Total workouts completed
    pub workouts_completed: u32,
    /// Total time spent working out in minutes
    pub total_minutes: u32,
    /// Total volume lifted in kg
    pub total_volume_kg: f64,
    /// Total sets completed
    pub total_sets: u32,
    /// Total calories burned
    pub total_calories: u32,
    /// Average workout duration in minutes
    pub avg_workout_minutes: u32,
    /// Average sets per workout
    pub avg_sets_per_workout: f64,
}

/// Workout streak information.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutStreak {
    /// Current consecutive days
    pub current_streak: u32,
    /// Longest ever streak
    pub longest_streak: u32,
    /// Whether user worked out today
    pub worked_out_today: bool,
    /// Days since last workout (0 if today)
    pub days_since_last_workout: u32,
}

impl Default for WorkoutStreak {
    fn default() -> Self {
        Self {
            current_streak: 0,
            longest_streak: 0,
            worked_out_today: false,
            days_since_last_workout: 0,
        }
    }
}

/// Progress data for a specific exercise.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ExerciseProgress {
    pub exercise_id: String,
    pub exercise_name: String,
    /// Estimated 1RM over time (timestamp, value)
    pub one_rm_history: Vec<ProgressPoint>,
    /// Volume over time
    pub volume_history: Vec<ProgressPoint>,
    /// Current estimated 1RM
    pub current_one_rm: f64,
    /// Best ever 1RM
    pub best_one_rm: f64,
    /// Total times performed
    pub times_performed: u32,
}

/// A single data point for progress tracking.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ProgressPoint {
    /// Unix timestamp
    pub timestamp: i64,
    /// The value at this point
    pub value: f64,
}

/// Weekly goal progress.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WeeklyGoals {
    /// Target workouts per week
    pub target_workouts: u32,
    /// Completed workouts this week
    pub completed_workouts: u32,
    /// Target active minutes per week
    pub target_minutes: u32,
    /// Completed active minutes this week
    pub completed_minutes: u32,
    /// Target calories per week
    pub target_calories: u32,
    /// Burned calories this week
    pub burned_calories: u32,
}

impl Default for WeeklyGoals {
    fn default() -> Self {
        Self {
            target_workouts: 4,
            completed_workouts: 0,
            target_minutes: 150,
            completed_minutes: 0,
            target_calories: 1000,
            burned_calories: 0,
        }
    }
}

impl WeeklyGoals {
    /// Calculate workout goal progress as percentage (0-100).
    pub fn workout_progress(&self) -> f64 {
        if self.target_workouts == 0 {
            return 100.0;
        }
        ((self.completed_workouts as f64 / self.target_workouts as f64) * 100.0).min(100.0)
    }

    /// Calculate minutes goal progress as percentage (0-100).
    pub fn minutes_progress(&self) -> f64 {
        if self.target_minutes == 0 {
            return 100.0;
        }
        ((self.completed_minutes as f64 / self.target_minutes as f64) * 100.0).min(100.0)
    }

    /// Calculate calories goal progress as percentage (0-100).
    pub fn calories_progress(&self) -> f64 {
        if self.target_calories == 0 {
            return 100.0;
        }
        ((self.burned_calories as f64 / self.target_calories as f64) * 100.0).min(100.0)
    }
}

/// Factory function to create weekly goals (UniFFI-compatible).
#[uniffi::export]
pub fn create_weekly_goals() -> WeeklyGoals {
    WeeklyGoals::default()
}

/// Calculate weekly goals progress percentages.
#[uniffi::export]
pub fn calculate_goals_progress(goals: &WeeklyGoals) -> Vec<f64> {
    vec![
        goals.workout_progress(),
        goals.minutes_progress(),
        goals.calories_progress(),
    ]
}

/// Muscle group workout frequency for the week.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct MuscleFrequency {
    pub muscle_group: String,
    pub times_trained: u32,
    pub last_trained_timestamp: Option<i64>,
}

/// Overall user statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize, uniffi::Record)]
pub struct UserStats {
    /// Total workouts ever
    pub total_workouts: u32,
    /// Total time spent (minutes)
    pub total_minutes: u32,
    /// Total volume lifted (kg)
    pub total_volume_kg: f64,
    /// Total sets completed
    pub total_sets: u32,
    /// Total calories burned
    pub total_calories: u32,
    /// Date of first workout (timestamp)
    pub first_workout_timestamp: Option<i64>,
    /// Number of personal records
    pub personal_records_count: u32,
}

/// Aggregate statistics from a list of workouts.
#[uniffi::export]
pub fn aggregate_workout_stats(workouts: Vec<Workout>) -> UserStats {
    let mut stats = UserStats::default();

    for workout in &workouts {
        stats.total_workouts += 1;
        stats.total_minutes += (workout.duration_seconds() / 60) as u32;
        stats.total_volume_kg += workout.total_volume_kg();
        stats.total_sets += workout.total_sets();
        stats.total_calories += workout.calories_burned.unwrap_or(0);

        if stats.first_workout_timestamp.is_none()
            || Some(workout.started_at) < stats.first_workout_timestamp
        {
            stats.first_workout_timestamp = Some(workout.started_at);
        }
    }

    stats
}

/// Calculate period metrics from workouts.
#[uniffi::export]
pub fn calculate_period_metrics(workouts: Vec<Workout>) -> PeriodMetrics {
    if workouts.is_empty() {
        return PeriodMetrics::default();
    }

    let total_minutes: u32 = workouts
        .iter()
        .map(|w| (w.duration_seconds() / 60) as u32)
        .sum();

    let total_sets: u32 = workouts.iter().map(|w| w.total_sets()).sum();
    let total_volume: f64 = workouts.iter().map(|w| w.total_volume_kg()).sum();
    let total_calories: u32 = workouts.iter().map(|w| w.calories_burned.unwrap_or(0)).sum();
    let workout_count = workouts.len() as u32;

    PeriodMetrics {
        workouts_completed: workout_count,
        total_minutes,
        total_volume_kg: total_volume,
        total_sets,
        total_calories,
        avg_workout_minutes: if workout_count > 0 {
            total_minutes / workout_count
        } else {
            0
        },
        avg_sets_per_workout: if workout_count > 0 {
            total_sets as f64 / workout_count as f64
        } else {
            0.0
        },
    }
}

/// Achievement definitions.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub is_unlocked: bool,
    pub unlocked_at: Option<i64>,
    pub progress: f64,
    pub target: f64,
}

/// Check achievements based on user stats.
#[uniffi::export]
pub fn check_achievements(stats: &UserStats) -> Vec<Achievement> {
    vec![
        Achievement {
            id: "first_workout".to_string(),
            name: "Getting Started".to_string(),
            description: "Complete your first workout".to_string(),
            icon: "dumbbell".to_string(),
            is_unlocked: stats.total_workouts >= 1,
            unlocked_at: None,
            progress: stats.total_workouts.min(1) as f64,
            target: 1.0,
        },
        Achievement {
            id: "ten_workouts".to_string(),
            name: "Dedicated".to_string(),
            description: "Complete 10 workouts".to_string(),
            icon: "flame".to_string(),
            is_unlocked: stats.total_workouts >= 10,
            unlocked_at: None,
            progress: stats.total_workouts.min(10) as f64,
            target: 10.0,
        },
        Achievement {
            id: "fifty_workouts".to_string(),
            name: "Committed".to_string(),
            description: "Complete 50 workouts".to_string(),
            icon: "trophy".to_string(),
            is_unlocked: stats.total_workouts >= 50,
            unlocked_at: None,
            progress: stats.total_workouts.min(50) as f64,
            target: 50.0,
        },
        Achievement {
            id: "hundred_workouts".to_string(),
            name: "Century Club".to_string(),
            description: "Complete 100 workouts".to_string(),
            icon: "star".to_string(),
            is_unlocked: stats.total_workouts >= 100,
            unlocked_at: None,
            progress: stats.total_workouts.min(100) as f64,
            target: 100.0,
        },
        Achievement {
            id: "thousand_sets".to_string(),
            name: "Set Master".to_string(),
            description: "Complete 1,000 sets".to_string(),
            icon: "repeat".to_string(),
            is_unlocked: stats.total_sets >= 1000,
            unlocked_at: None,
            progress: stats.total_sets.min(1000) as f64,
            target: 1000.0,
        },
        Achievement {
            id: "ton_club".to_string(),
            name: "Ton Club".to_string(),
            description: "Lift 1,000 kg total volume".to_string(),
            icon: "weight".to_string(),
            is_unlocked: stats.total_volume_kg >= 1000.0,
            unlocked_at: None,
            progress: stats.total_volume_kg.min(1000.0),
            target: 1000.0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekly_goals_progress() {
        let mut goals = WeeklyGoals::default();
        goals.completed_workouts = 2;
        goals.completed_minutes = 75;
        goals.burned_calories = 500;

        assert_eq!(goals.workout_progress(), 50.0);
        assert_eq!(goals.minutes_progress(), 50.0);
        assert_eq!(goals.calories_progress(), 50.0);
    }

    #[test]
    fn test_aggregate_stats() {
        let mut workout1 = Workout::new("Test 1".to_string());
        workout1.calories_burned = Some(100);

        let mut workout2 = Workout::new("Test 2".to_string());
        workout2.calories_burned = Some(150);

        let stats = aggregate_workout_stats(vec![workout1, workout2]);

        assert_eq!(stats.total_workouts, 2);
        assert_eq!(stats.total_calories, 250);
    }

    #[test]
    fn test_achievements() {
        let mut stats = UserStats::default();
        stats.total_workouts = 15;
        stats.total_sets = 500;
        stats.total_volume_kg = 5000.0;

        let achievements = check_achievements(&stats);

        let first_workout = achievements.iter().find(|a| a.id == "first_workout").unwrap();
        assert!(first_workout.is_unlocked);

        let ten_workouts = achievements.iter().find(|a| a.id == "ten_workouts").unwrap();
        assert!(ten_workouts.is_unlocked);

        let fifty_workouts = achievements.iter().find(|a| a.id == "fifty_workouts").unwrap();
        assert!(!fifty_workouts.is_unlocked);
        assert_eq!(fifty_workouts.progress, 15.0);
    }
}
