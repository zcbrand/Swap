//! Workout Session State Machine
//!
//! Manages the lifecycle of a workout session with clear state transitions.
//! Provides real-time updates without requiring user action (HIG requirement).

use crate::error::{Result, SwapError};
use crate::models::{EntityId, Exercise, Workout, WeightUnit};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

/// Current state of a workout session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum SessionState {
    /// No active session
    Idle,
    /// Workout in progress
    Active,
    /// Workout paused (rest timer, etc.)
    Paused,
    /// Rest timer counting down
    Resting,
    /// Workout completed, showing summary
    Completed,
}

/// Events that can occur during a workout session.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Enum)]
pub enum SessionEvent {
    /// User started a new workout
    Started { workout_name: String },
    /// User paused the workout
    Paused,
    /// User resumed the workout
    Resumed,
    /// User completed a set
    SetCompleted {
        exercise_id: String,
        weight: f64,
        reps: u32,
    },
    /// Rest timer started
    RestStarted { duration_seconds: u32 },
    /// Rest timer completed
    RestCompleted,
    /// User ended the workout
    Ended,
    /// User discarded the workout
    Discarded,
}

/// Real-time metrics for the current session (HIG: updates without user action).
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct SessionMetrics {
    /// Total elapsed time in seconds
    pub elapsed_seconds: i64,
    /// Time spent actively working (excluding rest)
    pub active_seconds: i64,
    /// Total volume lifted in kg
    pub total_volume_kg: f64,
    /// Number of completed sets
    pub completed_sets: u32,
    /// Number of completed exercises
    pub completed_exercises: u32,
    /// Estimated calories burned
    pub estimated_calories: u32,
    /// Current rest timer remaining (0 if not resting)
    pub rest_remaining_seconds: u32,
}

/// Log entry for session events.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct SessionEventLog {
    pub event: SessionEvent,
    pub timestamp: i64,
}

/// Internal state for WorkoutSession (behind RwLock for interior mutability).
#[derive(Debug)]
struct SessionInner {
    state: SessionState,
    workout: Option<Workout>,
    current_exercise_index: usize,
    rest_timer_start: Option<i64>,
    rest_duration: u32,
    pause_start: Option<i64>,
    total_pause_duration: i64,
    events: Vec<SessionEventLog>,
}

impl Default for SessionInner {
    fn default() -> Self {
        Self {
            state: SessionState::Idle,
            workout: None,
            current_exercise_index: 0,
            rest_timer_start: None,
            rest_duration: 0,
            pause_start: None,
            total_pause_duration: 0,
            events: Vec::new(),
        }
    }
}

/// A workout session manager with state machine.
/// Uses interior mutability to work with UniFFI's Arc wrapper.
#[derive(uniffi::Object)]
pub struct WorkoutSession {
    inner: RwLock<SessionInner>,
}

#[uniffi::export]
impl WorkoutSession {
    /// Create a new workout session.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(SessionInner::default()),
        }
    }

    /// Get the current session state.
    pub fn state(&self) -> SessionState {
        self.inner.read().unwrap().state
    }

    /// Check if a workout is active.
    pub fn is_active(&self) -> bool {
        let inner = self.inner.read().unwrap();
        matches!(
            inner.state,
            SessionState::Active | SessionState::Paused | SessionState::Resting
        )
    }

    /// Start a new workout.
    pub fn start(&self, workout_name: String) -> Result<()> {
        let mut inner = self.inner.write().unwrap();

        if matches!(
            inner.state,
            SessionState::Active | SessionState::Paused | SessionState::Resting
        ) {
            return Err(SwapError::InvalidWorkout {
                message: "A workout is already in progress".to_string(),
            });
        }

        let workout = Workout::new(workout_name.clone());
        inner.workout = Some(workout);
        inner.state = SessionState::Active;
        inner.current_exercise_index = 0;
        inner.total_pause_duration = 0;

        inner.events.push(SessionEventLog {
            event: SessionEvent::Started { workout_name },
            timestamp: Utc::now().timestamp(),
        });
        Ok(())
    }

    /// Start a workout from an existing workout object.
    pub fn start_with_workout(&self, workout: Workout) -> Result<()> {
        let mut inner = self.inner.write().unwrap();

        if matches!(
            inner.state,
            SessionState::Active | SessionState::Paused | SessionState::Resting
        ) {
            return Err(SwapError::InvalidWorkout {
                message: "A workout is already in progress".to_string(),
            });
        }

        let name = workout.name.clone();
        inner.workout = Some(workout);
        inner.state = SessionState::Active;
        inner.current_exercise_index = 0;
        inner.total_pause_duration = 0;

        inner.events.push(SessionEventLog {
            event: SessionEvent::Started { workout_name: name },
            timestamp: Utc::now().timestamp(),
        });
        Ok(())
    }

    /// Pause the workout.
    pub fn pause(&self) -> Result<()> {
        let mut inner = self.inner.write().unwrap();

        if inner.state != SessionState::Active && inner.state != SessionState::Resting {
            return Err(SwapError::InvalidWorkout {
                message: "Cannot pause: workout is not active".to_string(),
            });
        }

        inner.pause_start = Some(Utc::now().timestamp());
        inner.state = SessionState::Paused;
        inner.events.push(SessionEventLog {
            event: SessionEvent::Paused,
            timestamp: Utc::now().timestamp(),
        });
        Ok(())
    }

    /// Resume a paused workout.
    pub fn resume(&self) -> Result<()> {
        let mut inner = self.inner.write().unwrap();

        if inner.state != SessionState::Paused {
            return Err(SwapError::InvalidWorkout {
                message: "Cannot resume: workout is not paused".to_string(),
            });
        }

        if let Some(pause_start) = inner.pause_start {
            inner.total_pause_duration += Utc::now().timestamp() - pause_start;
        }
        inner.pause_start = None;
        inner.state = SessionState::Active;
        inner.events.push(SessionEventLog {
            event: SessionEvent::Resumed,
            timestamp: Utc::now().timestamp(),
        });
        Ok(())
    }

    /// Add an exercise to the current workout.
    pub fn add_exercise(&self, exercise: Exercise) -> Result<EntityId> {
        let mut inner = self.inner.write().unwrap();

        let workout = inner.workout.as_mut().ok_or(SwapError::InvalidWorkout {
            message: "No active workout".to_string(),
        })?;

        let id = workout.add_exercise(exercise);
        Ok(id)
    }

    /// Log a completed set (optimized for <5 second logging time).
    pub fn log_set(
        &self,
        exercise_id: String,
        weight: f64,
        weight_unit: WeightUnit,
        reps: u32,
    ) -> Result<EntityId> {
        let mut inner = self.inner.write().unwrap();

        let workout = inner.workout.as_mut().ok_or(SwapError::InvalidWorkout {
            message: "No active workout".to_string(),
        })?;

        // Find the exercise
        let exercise = workout
            .exercises
            .iter_mut()
            .find(|e| e.id.value == exercise_id)
            .ok_or(SwapError::ExerciseNotFound {
                exercise_id: exercise_id.clone(),
            })?;

        // Add and complete the set
        let set_id = exercise.add_set(weight, weight_unit, reps);
        if let Some(set) = exercise.sets.last_mut() {
            set.complete();
        }

        inner.events.push(SessionEventLog {
            event: SessionEvent::SetCompleted {
                exercise_id,
                weight,
                reps,
            },
            timestamp: Utc::now().timestamp(),
        });

        Ok(set_id)
    }

    /// Quick log: log a set with same weight/reps as previous (1 tap).
    pub fn quick_log(&self, exercise_id: String) -> Result<EntityId> {
        // First, get the last set's data
        let (weight, weight_unit, reps) = {
            let inner = self.inner.read().unwrap();
            let workout = inner.workout.as_ref().ok_or(SwapError::InvalidWorkout {
                message: "No active workout".to_string(),
            })?;

            let exercise = workout
                .exercises
                .iter()
                .find(|e| e.id.value == exercise_id)
                .ok_or(SwapError::ExerciseNotFound {
                    exercise_id: exercise_id.clone(),
                })?;

            let last_set = exercise.sets.last().ok_or(SwapError::InvalidWorkout {
                message: "No previous set to copy".to_string(),
            })?;

            (last_set.weight, last_set.weight_unit, last_set.reps)
        };

        // Now log the new set
        self.log_set(exercise_id, weight, weight_unit, reps)
    }

    /// Start a rest timer.
    pub fn start_rest(&self, duration_seconds: u32) {
        let mut inner = self.inner.write().unwrap();
        inner.rest_timer_start = Some(Utc::now().timestamp());
        inner.rest_duration = duration_seconds;
        inner.state = SessionState::Resting;
        inner.events.push(SessionEventLog {
            event: SessionEvent::RestStarted { duration_seconds },
            timestamp: Utc::now().timestamp(),
        });
    }

    /// Complete the rest timer (or skip it).
    pub fn end_rest(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.rest_timer_start = None;
        inner.rest_duration = 0;
        inner.state = SessionState::Active;
        inner.events.push(SessionEventLog {
            event: SessionEvent::RestCompleted,
            timestamp: Utc::now().timestamp(),
        });
    }

    /// Get remaining rest time in seconds.
    pub fn rest_remaining(&self) -> u32 {
        let inner = self.inner.read().unwrap();

        if inner.state != SessionState::Resting {
            return 0;
        }

        if let Some(start) = inner.rest_timer_start {
            let elapsed = Utc::now().timestamp() - start;
            let remaining = inner.rest_duration as i64 - elapsed;
            return remaining.max(0) as u32;
        }

        0
    }

    /// End the workout and get the completed workout.
    pub fn end_workout(&self) -> Result<Workout> {
        let mut inner = self.inner.write().unwrap();

        if !matches!(
            inner.state,
            SessionState::Active | SessionState::Paused | SessionState::Resting
        ) {
            return Err(SwapError::InvalidWorkout {
                message: "No active workout to end".to_string(),
            });
        }

        inner.events.push(SessionEventLog {
            event: SessionEvent::Ended,
            timestamp: Utc::now().timestamp(),
        });

        let mut workout = inner.workout.take().unwrap();
        workout.complete();

        // Calculate estimated calories (rough estimate: 3-5 cal per set)
        let total_sets = workout.total_sets();
        workout.calories_burned =
            Some((total_sets * 4) as u32 + (workout.duration_seconds() / 60) as u32);

        inner.state = SessionState::Completed;

        // Reset internal state
        inner.current_exercise_index = 0;
        inner.rest_timer_start = None;
        inner.rest_duration = 0;
        inner.pause_start = None;
        inner.total_pause_duration = 0;
        inner.state = SessionState::Idle;

        Ok(workout)
    }

    /// Discard the current workout without saving.
    pub fn discard(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.events.push(SessionEventLog {
            event: SessionEvent::Discarded,
            timestamp: Utc::now().timestamp(),
        });
        inner.workout = None;

        // Reset internal state
        inner.current_exercise_index = 0;
        inner.rest_timer_start = None;
        inner.rest_duration = 0;
        inner.pause_start = None;
        inner.total_pause_duration = 0;
        inner.state = SessionState::Idle;
    }

    /// Get current session metrics (real-time updates).
    pub fn metrics(&self) -> SessionMetrics {
        let inner = self.inner.read().unwrap();
        let now = Utc::now().timestamp();

        let (elapsed, active, volume, sets, exercises) = if let Some(workout) = &inner.workout {
            let elapsed = now - workout.started_at - inner.total_pause_duration;
            let pause_time = inner.pause_start.map(|p| now - p).unwrap_or(0);
            let active = elapsed - pause_time;
            let volume = workout.total_volume_kg();
            let sets = workout.total_sets();
            let exercises = workout
                .exercises
                .iter()
                .filter(|e| e.completed_sets() > 0)
                .count() as u32;

            (elapsed, active, volume, sets, exercises)
        } else {
            (0, 0, 0.0, 0, 0)
        };

        let rest_remaining = if inner.state == SessionState::Resting {
            if let Some(start) = inner.rest_timer_start {
                let elapsed = now - start;
                (inner.rest_duration as i64 - elapsed).max(0) as u32
            } else {
                0
            }
        } else {
            0
        };

        // Estimate calories: ~0.05 cal per kg volume + 4 cal per minute active
        let estimated_calories = ((volume * 0.05) + (active as f64 / 60.0 * 4.0)) as u32;

        SessionMetrics {
            elapsed_seconds: elapsed,
            active_seconds: active,
            total_volume_kg: volume,
            completed_sets: sets,
            completed_exercises: exercises,
            estimated_calories,
            rest_remaining_seconds: rest_remaining,
        }
    }

    /// Get the current workout (if any).
    pub fn current_workout(&self) -> Option<Workout> {
        self.inner.read().unwrap().workout.clone()
    }

    /// Get event history.
    pub fn event_history(&self) -> Vec<SessionEventLog> {
        self.inner.read().unwrap().events.clone()
    }

    /// Move to the next exercise.
    pub fn next_exercise(&self) {
        let mut inner = self.inner.write().unwrap();
        if let Some(workout) = &inner.workout {
            if inner.current_exercise_index < workout.exercises.len().saturating_sub(1) {
                inner.current_exercise_index += 1;
            }
        }
    }

    /// Move to the previous exercise.
    pub fn previous_exercise(&self) {
        let mut inner = self.inner.write().unwrap();
        if inner.current_exercise_index > 0 {
            inner.current_exercise_index -= 1;
        }
    }

    /// Get current exercise index.
    pub fn current_exercise_index(&self) -> u32 {
        self.inner.read().unwrap().current_exercise_index as u32
    }
}

impl Default for WorkoutSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of a completed workout (for end-of-workout screen).
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct WorkoutSummary {
    pub workout_id: String,
    pub workout_name: String,
    pub duration_seconds: i64,
    pub total_volume_kg: f64,
    pub total_sets: u32,
    pub total_exercises: u32,
    pub calories_burned: u32,
    pub personal_records: Vec<PersonalRecord>,
    pub can_sync_to_health: bool,
}

/// A personal record achieved during the workout.
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct PersonalRecord {
    pub exercise_name: String,
    pub record_type: String,
    pub value: f64,
    pub previous_value: Option<f64>,
}

/// Create a workout summary from a completed workout.
#[uniffi::export]
pub fn create_workout_summary(workout: &Workout) -> WorkoutSummary {
    WorkoutSummary {
        workout_id: workout.id.value.clone(),
        workout_name: workout.name.clone(),
        duration_seconds: workout.duration_seconds(),
        total_volume_kg: workout.total_volume_kg(),
        total_sets: workout.total_sets(),
        total_exercises: workout.exercises.len() as u32,
        calories_burned: workout.calories_burned.unwrap_or(0),
        personal_records: Vec::new(), // TODO: Implement PR detection
        can_sync_to_health: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ExerciseType, MuscleGroup};

    fn create_test_exercise() -> Exercise {
        Exercise::new(
            "Bench Press".to_string(),
            ExerciseType::Resistance,
            MuscleGroup::Chest,
        )
    }

    #[test]
    fn test_session_lifecycle() {
        let session = WorkoutSession::new();
        assert_eq!(session.state(), SessionState::Idle);

        session.start("Test Workout".to_string()).unwrap();
        assert_eq!(session.state(), SessionState::Active);
        assert!(session.is_active());

        session.pause().unwrap();
        assert_eq!(session.state(), SessionState::Paused);

        session.resume().unwrap();
        assert_eq!(session.state(), SessionState::Active);

        let workout = session.end_workout().unwrap();
        assert!(workout.completed_at.is_some());
        assert_eq!(session.state(), SessionState::Idle);
    }

    #[test]
    fn test_log_set() {
        let session = WorkoutSession::new();
        session.start("Test".to_string()).unwrap();

        let exercise = create_test_exercise();
        let exercise_id = session.add_exercise(exercise).unwrap();

        session
            .log_set(exercise_id.value.clone(), 100.0, WeightUnit::Pounds, 10)
            .unwrap();

        let metrics = session.metrics();
        assert_eq!(metrics.completed_sets, 1);
    }

    #[test]
    fn test_quick_log() {
        let session = WorkoutSession::new();
        session.start("Test".to_string()).unwrap();

        let exercise = create_test_exercise();
        let exercise_id = session.add_exercise(exercise).unwrap();

        // First set
        session
            .log_set(exercise_id.value.clone(), 100.0, WeightUnit::Pounds, 10)
            .unwrap();

        // Quick log should copy the previous set
        session.quick_log(exercise_id.value).unwrap();

        let metrics = session.metrics();
        assert_eq!(metrics.completed_sets, 2);
    }

    #[test]
    fn test_rest_timer() {
        let session = WorkoutSession::new();
        session.start("Test".to_string()).unwrap();

        session.start_rest(90);
        assert_eq!(session.state(), SessionState::Resting);

        // Rest remaining should be close to 90 (might be slightly less due to execution time)
        let remaining = session.rest_remaining();
        assert!(remaining <= 90);
        assert!(remaining >= 85);

        session.end_rest();
        assert_eq!(session.state(), SessionState::Active);
        assert_eq!(session.rest_remaining(), 0);
    }
}
