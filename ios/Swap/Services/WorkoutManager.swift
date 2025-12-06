// WorkoutManager.swift
// Manages workout state and bridges to Rust core via UniFFI

import Foundation
import Combine

/// Manages the active workout session.
/// Provides real-time metrics updates without requiring user action (HIG requirement).
@MainActor
class WorkoutManager: ObservableObject {
    // MARK: - Published Properties

    @Published var isWorkoutActive = false
    @Published var currentWorkoutName = ""
    @Published var currentExerciseIndex = 0
    @Published var exercises: [WorkoutExerciseUI] = []
    @Published var elapsedTime: TimeInterval = 0
    @Published var totalVolume: Double = 0
    @Published var completedSets: Int = 0
    @Published var estimatedCalories: Int = 0
    @Published var isPaused = false
    @Published var isResting = false
    @Published var restTimeRemaining: Int = 0
    @Published var sessionState: SessionStateUI = .idle

    // MARK: - Private Properties

    private var timer: Timer?
    private var restTimer: Timer?
    private var startTime: Date?
    private var pausedDuration: TimeInterval = 0
    private var pauseStartTime: Date?

    // Default rest time in seconds
    var defaultRestTime: Int = 90

    // MARK: - Workout Lifecycle

    /// Start a new workout.
    func startWorkout(name: String, exercises: [ExerciseUI] = []) {
        currentWorkoutName = name
        self.exercises = exercises.enumerated().map { index, exercise in
            WorkoutExerciseUI(
                id: UUID(),
                exercise: exercise,
                sets: [],
                order: index
            )
        }
        startTime = Date()
        isWorkoutActive = true
        isPaused = false
        sessionState = .active
        elapsedTime = 0
        totalVolume = 0
        completedSets = 0
        estimatedCalories = 0
        currentExerciseIndex = 0

        startTimer()
        provideHapticFeedback(.medium)
    }

    /// Pause the workout.
    func pauseWorkout() {
        guard isWorkoutActive, !isPaused else { return }
        isPaused = true
        sessionState = .paused
        pauseStartTime = Date()
        timer?.invalidate()
        provideHapticFeedback(.light)
    }

    /// Resume the workout.
    func resumeWorkout() {
        guard isWorkoutActive, isPaused else { return }
        if let pauseStart = pauseStartTime {
            pausedDuration += Date().timeIntervalSince(pauseStart)
        }
        isPaused = false
        sessionState = .active
        pauseStartTime = nil
        startTimer()
        provideHapticFeedback(.light)
    }

    /// End the workout and return summary.
    func endWorkout() -> WorkoutSummaryUI {
        timer?.invalidate()
        restTimer?.invalidate()

        let summary = WorkoutSummaryUI(
            id: UUID(),
            name: currentWorkoutName,
            duration: elapsedTime,
            totalVolume: totalVolume,
            totalSets: completedSets,
            totalExercises: exercises.filter { !$0.sets.isEmpty }.count,
            caloriesBurned: estimatedCalories,
            completedAt: Date()
        )

        // Provide strong haptic feedback for workout completion
        provideHapticFeedback(.heavy)

        // Reset state
        resetSession()
        sessionState = .completed

        return summary
    }

    /// Discard the workout without saving.
    func discardWorkout() {
        timer?.invalidate()
        restTimer?.invalidate()
        resetSession()
        provideHapticFeedback(.light)
    }

    // MARK: - Set Logging (Optimized for <5 second logging time)

    /// Log a completed set. Designed for minimal taps.
    func logSet(weight: Double, reps: Int, rpe: Double? = nil) {
        guard currentExerciseIndex < exercises.count else { return }

        let setNumber = exercises[currentExerciseIndex].sets.count + 1
        let newSet = WorkoutSetUI(
            id: UUID(),
            setNumber: setNumber,
            weight: weight,
            reps: reps,
            rpe: rpe,
            isCompleted: true,
            completedAt: Date()
        )

        exercises[currentExerciseIndex].sets.append(newSet)
        completedSets += 1
        totalVolume += weight * Double(reps)
        estimatedCalories = calculateCalories()

        provideHapticFeedback(.medium)

        // Auto-start rest timer
        startRestTimer()
    }

    /// Quick log: repeat the last set (1 tap operation).
    func quickLog() {
        guard currentExerciseIndex < exercises.count,
              let lastSet = exercises[currentExerciseIndex].sets.last else { return }

        logSet(weight: lastSet.weight, reps: lastSet.reps, rpe: lastSet.rpe)
    }

    /// Log a set and move to next exercise.
    func logSetAndNext(weight: Double, reps: Int) {
        logSet(weight: weight, reps: reps)
        if currentExerciseIndex < exercises.count - 1 {
            currentExerciseIndex += 1
        }
    }

    // MARK: - Exercise Navigation

    func nextExercise() {
        guard currentExerciseIndex < exercises.count - 1 else { return }
        currentExerciseIndex += 1
        provideHapticFeedback(.selection)
    }

    func previousExercise() {
        guard currentExerciseIndex > 0 else { return }
        currentExerciseIndex -= 1
        provideHapticFeedback(.selection)
    }

    func selectExercise(at index: Int) {
        guard index >= 0, index < exercises.count else { return }
        currentExerciseIndex = index
        provideHapticFeedback(.selection)
    }

    // MARK: - Rest Timer

    func startRestTimer(duration: Int? = nil) {
        let restDuration = duration ?? defaultRestTime
        restTimeRemaining = restDuration
        isResting = true
        sessionState = .resting

        restTimer?.invalidate()
        restTimer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { [weak self] _ in
            Task { @MainActor in
                self?.updateRestTimer()
            }
        }
    }

    func skipRest() {
        restTimer?.invalidate()
        restTimeRemaining = 0
        isResting = false
        sessionState = .active
        provideHapticFeedback(.light)
    }

    func addRestTime(_ seconds: Int) {
        restTimeRemaining += seconds
    }

    // MARK: - Private Helpers

    private func startTimer() {
        timer?.invalidate()
        timer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { [weak self] _ in
            Task { @MainActor in
                self?.updateElapsedTime()
            }
        }
    }

    private func updateElapsedTime() {
        guard let start = startTime, !isPaused else { return }
        elapsedTime = Date().timeIntervalSince(start) - pausedDuration
        estimatedCalories = calculateCalories()
    }

    private func updateRestTimer() {
        guard isResting else { return }

        if restTimeRemaining > 0 {
            restTimeRemaining -= 1
        } else {
            restTimer?.invalidate()
            isResting = false
            sessionState = .active
            provideHapticFeedback(.heavy) // Alert user rest is over
        }
    }

    private func calculateCalories() -> Int {
        // Rough estimate: 4 cal per set + 0.1 cal per kg volume + 4 cal per active minute
        let setCalories = completedSets * 4
        let volumeCalories = Int(totalVolume * 0.05)
        let timeCalories = Int(elapsedTime / 60) * 4
        return setCalories + volumeCalories + timeCalories
    }

    private func resetSession() {
        isWorkoutActive = false
        currentWorkoutName = ""
        exercises = []
        currentExerciseIndex = 0
        elapsedTime = 0
        totalVolume = 0
        completedSets = 0
        estimatedCalories = 0
        isPaused = false
        isResting = false
        restTimeRemaining = 0
        startTime = nil
        pausedDuration = 0
        pauseStartTime = nil
        timer?.invalidate()
        restTimer?.invalidate()
        sessionState = .idle
    }

    private func provideHapticFeedback(_ style: HapticStyle) {
        #if os(iOS)
        let generator: UIImpactFeedbackGenerator
        switch style {
        case .light:
            generator = UIImpactFeedbackGenerator(style: .light)
        case .medium:
            generator = UIImpactFeedbackGenerator(style: .medium)
        case .heavy:
            generator = UIImpactFeedbackGenerator(style: .heavy)
        case .selection:
            let selectionGenerator = UISelectionFeedbackGenerator()
            selectionGenerator.selectionChanged()
            return
        }
        generator.impactOccurred()
        #endif
    }
}

// MARK: - Supporting Types

enum SessionStateUI {
    case idle
    case active
    case paused
    case resting
    case completed
}

enum HapticStyle {
    case light
    case medium
    case heavy
    case selection
}

// MARK: - UI Model Types

struct ExerciseUI: Identifiable, Hashable {
    let id: UUID
    let name: String
    let muscleGroup: String
    let exerciseType: String
}

struct WorkoutExerciseUI: Identifiable {
    let id: UUID
    let exercise: ExerciseUI
    var sets: [WorkoutSetUI]
    let order: Int
}

struct WorkoutSetUI: Identifiable {
    let id: UUID
    let setNumber: Int
    let weight: Double
    let reps: Int
    let rpe: Double?
    let isCompleted: Bool
    let completedAt: Date?
}

struct WorkoutSummaryUI: Identifiable {
    let id: UUID
    let name: String
    let duration: TimeInterval
    let totalVolume: Double
    let totalSets: Int
    let totalExercises: Int
    let caloriesBurned: Int
    let completedAt: Date
}
