//! Core data models for workout tracking.
//!
//! These models are exposed to iOS and Android via UniFFI bindings.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for entities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, uniffi::Record)]
pub struct EntityId {
    pub value: String,
}

impl EntityId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
        }
    }

    pub fn from_string(s: String) -> Self {
        Self { value: s }
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

/// Weight unit for exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum WeightUnit {
    Kilograms,
    Pounds,
}

impl WeightUnit {
    /// Convert a weight to kilograms.
    pub fn to_kg(&self, weight: f64) -> f64 {
        match self {
            WeightUnit::Kilograms => weight,
            WeightUnit::Pounds => weight * 0.453592,
        }
    }

    /// Convert a weight from kilograms.
    pub fn from_kg(&self, kg: f64) -> f64 {
        match self {
            WeightUnit::Kilograms => kg,
            WeightUnit::Pounds => kg / 0.453592,
        }
    }
}

/// Muscle group categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum MuscleGroup {
    Chest,
    Back,
    Shoulders,
    Biceps,
    Triceps,
    Forearms,
    Core,
    Quadriceps,
    Hamstrings,
    Glutes,
    Calves,
    FullBody,
}

/// Type of exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum ExerciseType {
    /// Barbell, dumbbell, machine exercises
    Resistance,
    /// Running, cycling, rowing
    Cardio,
    /// Bodyweight exercises
    Calisthenics,
    /// Stretching, yoga
    Flexibility,
}

/// An exercise definition (template).
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct Exercise {
    pub id: EntityId,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub primary_muscle: MuscleGroup,
    pub secondary_muscles: Vec<MuscleGroup>,
    pub instructions: Option<String>,
    pub is_custom: bool,
}

impl Exercise {
    pub fn new(
        name: String,
        exercise_type: ExerciseType,
        primary_muscle: MuscleGroup,
    ) -> Self {
        Self {
            id: EntityId::new(),
            name,
            exercise_type,
            primary_muscle,
            secondary_muscles: Vec::new(),
            instructions: None,
            is_custom: true,
        }
    }
}

/// A single set within a workout.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutSet {
    pub id: EntityId,
    pub set_number: u32,
    pub weight: f64,
    pub weight_unit: WeightUnit,
    pub reps: u32,
    pub rpe: Option<f64>,
    pub is_warmup: bool,
    pub is_dropset: bool,
    pub rest_seconds: Option<u32>,
    pub completed_at: Option<i64>,
    pub notes: Option<String>,
}

impl WorkoutSet {
    pub fn new(set_number: u32, weight: f64, weight_unit: WeightUnit, reps: u32) -> Self {
        Self {
            id: EntityId::new(),
            set_number,
            weight,
            weight_unit,
            reps,
            rpe: None,
            is_warmup: false,
            is_dropset: false,
            rest_seconds: None,
            completed_at: None,
            notes: None,
        }
    }

    /// Mark the set as completed with current timestamp.
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now().timestamp());
    }

    /// Check if the set is completed.
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Get weight in kilograms.
    pub fn weight_kg(&self) -> f64 {
        self.weight_unit.to_kg(self.weight)
    }
}

/// An exercise performed during a workout with its sets.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutExercise {
    pub id: EntityId,
    pub exercise: Exercise,
    pub sets: Vec<WorkoutSet>,
    pub order: u32,
    pub notes: Option<String>,
}

impl WorkoutExercise {
    pub fn new(exercise: Exercise, order: u32) -> Self {
        Self {
            id: EntityId::new(),
            exercise,
            sets: Vec::new(),
            order,
            notes: None,
        }
    }

    /// Add a set to this exercise.
    pub fn add_set(&mut self, weight: f64, weight_unit: WeightUnit, reps: u32) -> EntityId {
        let set_number = self.sets.len() as u32 + 1;
        let set = WorkoutSet::new(set_number, weight, weight_unit, reps);
        let id = set.id.clone();
        self.sets.push(set);
        id
    }

    /// Get total volume (weight × reps) for this exercise in kg.
    pub fn total_volume_kg(&self) -> f64 {
        self.sets
            .iter()
            .filter(|s| s.is_completed() && !s.is_warmup)
            .map(|s| s.weight_kg() * s.reps as f64)
            .sum()
    }

    /// Get the number of completed working sets.
    pub fn completed_sets(&self) -> u32 {
        self.sets
            .iter()
            .filter(|s| s.is_completed() && !s.is_warmup)
            .count() as u32
    }
}

/// A complete workout session.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct Workout {
    pub id: EntityId,
    pub name: String,
    pub exercises: Vec<WorkoutExercise>,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub notes: Option<String>,
    pub calories_burned: Option<u32>,
    pub synced_to_health: bool,
}

impl Workout {
    pub fn new(name: String) -> Self {
        Self {
            id: EntityId::new(),
            name,
            exercises: Vec::new(),
            started_at: Utc::now().timestamp(),
            completed_at: None,
            notes: None,
            calories_burned: None,
            synced_to_health: false,
        }
    }

    /// Add an exercise to the workout.
    pub fn add_exercise(&mut self, exercise: Exercise) -> EntityId {
        let order = self.exercises.len() as u32;
        let workout_exercise = WorkoutExercise::new(exercise, order);
        let id = workout_exercise.id.clone();
        self.exercises.push(workout_exercise);
        id
    }

    /// Mark the workout as completed.
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now().timestamp());
    }

    /// Check if workout is in progress.
    pub fn is_active(&self) -> bool {
        self.completed_at.is_none()
    }

    /// Get workout duration in seconds.
    pub fn duration_seconds(&self) -> i64 {
        let end_time = self.completed_at.unwrap_or_else(|| Utc::now().timestamp());
        end_time - self.started_at
    }

    /// Get total volume across all exercises in kg.
    pub fn total_volume_kg(&self) -> f64 {
        self.exercises.iter().map(|e| e.total_volume_kg()).sum()
    }

    /// Get total number of completed sets.
    pub fn total_sets(&self) -> u32 {
        self.exercises.iter().map(|e| e.completed_sets()).sum()
    }
}

/// Workout template for quick starts.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutTemplate {
    pub id: EntityId,
    pub name: String,
    pub exercises: Vec<Exercise>,
    pub estimated_duration_minutes: u32,
    pub is_custom: bool,
}

impl WorkoutTemplate {
    pub fn new(name: String) -> Self {
        Self {
            id: EntityId::new(),
            name,
            exercises: Vec::new(),
            estimated_duration_minutes: 45,
            is_custom: true,
        }
    }

    /// Create a new workout from this template.
    pub fn start_workout(&self) -> Workout {
        let mut workout = Workout::new(self.name.clone());
        for exercise in &self.exercises {
            workout.add_exercise(exercise.clone());
        }
        workout
    }
}

/// User preferences and settings.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct UserPreferences {
    pub default_weight_unit: WeightUnit,
    pub default_rest_seconds: u32,
    pub show_warmup_sets: bool,
    pub auto_start_rest_timer: bool,
    pub haptic_feedback_enabled: bool,
    pub health_sync_enabled: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_weight_unit: WeightUnit::Pounds,
            default_rest_seconds: 90,
            show_warmup_sets: true,
            auto_start_rest_timer: true,
            haptic_feedback_enabled: true,
            health_sync_enabled: true,
        }
    }
}

/// Factory function for creating user preferences (UniFFI-compatible).
#[uniffi::export]
pub fn create_default_preferences() -> UserPreferences {
    UserPreferences::default()
}

/// Factory function for creating a new exercise.
#[uniffi::export]
pub fn create_exercise(
    name: String,
    exercise_type: ExerciseType,
    primary_muscle: MuscleGroup,
) -> Exercise {
    Exercise::new(name, exercise_type, primary_muscle)
}

/// Factory function for creating a new workout.
#[uniffi::export]
pub fn create_workout(name: String) -> Workout {
    Workout::new(name)
}

/// Factory function for creating a new workout template.
#[uniffi::export]
pub fn create_workout_template(name: String) -> WorkoutTemplate {
    WorkoutTemplate::new(name)
}
