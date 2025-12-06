// ActiveWorkoutView.swift
// Main workout mode UI with HIG-compliant controls

import SwiftUI

/// Active workout view with distinct visual appearance.
/// HIG Requirements:
/// - Large, easily tapped controls (44pt minimum touch target)
/// - Distinct visual appearance during active workout
/// - Real-time metrics updates without user action
/// - Clear feedback when session starts or stops
struct ActiveWorkoutView: View {
    @EnvironmentObject var workoutManager: WorkoutManager
    @State private var showEndWorkoutConfirmation = false
    @State private var showExerciseList = false

    // Distinct workout mode colors
    private let workoutBackground = Color.black
    private let accentColor = Color.green

    var body: some View {
        ZStack {
            // Distinct dark background for workout mode
            workoutBackground.ignoresSafeArea()

            VStack(spacing: 0) {
                // Header with workout name and elapsed time
                WorkoutHeader(
                    workoutName: workoutManager.currentWorkoutName,
                    elapsedTime: workoutManager.elapsedTime,
                    isPaused: workoutManager.isPaused
                )

                // Real-time metrics strip
                MetricsStrip(
                    sets: workoutManager.completedSets,
                    volume: workoutManager.totalVolume,
                    calories: workoutManager.estimatedCalories
                )

                // Current exercise display
                if workoutManager.exercises.indices.contains(workoutManager.currentExerciseIndex) {
                    CurrentExerciseView(
                        exercise: workoutManager.exercises[workoutManager.currentExerciseIndex],
                        exerciseNumber: workoutManager.currentExerciseIndex + 1,
                        totalExercises: workoutManager.exercises.count
                    )
                } else {
                    EmptyExerciseView()
                }

                Spacer()

                // Rest timer overlay (if resting)
                if workoutManager.isResting {
                    RestTimerView(
                        timeRemaining: workoutManager.restTimeRemaining,
                        onSkip: { workoutManager.skipRest() },
                        onAddTime: { workoutManager.addRestTime(30) }
                    )
                }

                // Bottom control bar with large touch targets
                WorkoutControlBar(
                    isPaused: workoutManager.isPaused,
                    isResting: workoutManager.isResting,
                    onPause: { workoutManager.pauseWorkout() },
                    onResume: { workoutManager.resumeWorkout() },
                    onEnd: { showEndWorkoutConfirmation = true }
                )
            }
        }
        .preferredColorScheme(.dark)
        .confirmationDialog(
            "End Workout?",
            isPresented: $showEndWorkoutConfirmation,
            titleVisibility: .visible
        ) {
            Button("End & Save", role: .destructive) {
                _ = workoutManager.endWorkout()
            }
            Button("Discard", role: .destructive) {
                workoutManager.discardWorkout()
            }
            Button("Cancel", role: .cancel) {}
        } message: {
            Text("Your workout will be saved and can be synced with the Health app.")
        }
    }
}

// MARK: - Workout Header

struct WorkoutHeader: View {
    let workoutName: String
    let elapsedTime: TimeInterval
    let isPaused: Bool

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(workoutName)
                    .font(.headline)
                    .foregroundColor(.white)

                HStack(spacing: 4) {
                    if isPaused {
                        Image(systemName: "pause.fill")
                            .foregroundColor(.yellow)
                    }
                    Text(formatTime(elapsedTime))
                        .font(.system(.title2, design: .monospaced))
                        .foregroundColor(isPaused ? .yellow : .white)
                }
            }

            Spacer()
        }
        .padding()
        .background(Color.black.opacity(0.3))
    }

    private func formatTime(_ interval: TimeInterval) -> String {
        let hours = Int(interval) / 3600
        let minutes = (Int(interval) % 3600) / 60
        let seconds = Int(interval) % 60

        if hours > 0 {
            return String(format: "%d:%02d:%02d", hours, minutes, seconds)
        } else {
            return String(format: "%02d:%02d", minutes, seconds)
        }
    }
}

// MARK: - Metrics Strip

struct MetricsStrip: View {
    let sets: Int
    let volume: Double
    let calories: Int

    var body: some View {
        HStack(spacing: 0) {
            MetricItem(value: "\(sets)", label: "Sets", icon: "checkmark.circle.fill")
            Divider().background(Color.gray)
            MetricItem(value: formatVolume(volume), label: "Volume", icon: "scalemass.fill")
            Divider().background(Color.gray)
            MetricItem(value: "\(calories)", label: "Cal", icon: "flame.fill")
        }
        .frame(height: 70)
        .background(Color.gray.opacity(0.2))
    }

    private func formatVolume(_ volume: Double) -> String {
        if volume >= 1000 {
            return String(format: "%.1fk", volume / 1000)
        }
        return String(format: "%.0f", volume)
    }
}

struct MetricItem: View {
    let value: String
    let label: String
    let icon: String

    var body: some View {
        VStack(spacing: 4) {
            HStack(spacing: 4) {
                Image(systemName: icon)
                    .font(.caption)
                    .foregroundColor(.green)
                Text(value)
                    .font(.system(.title3, design: .rounded).bold())
                    .foregroundColor(.white)
            }
            Text(label)
                .font(.caption2)
                .foregroundColor(.gray)
        }
        .frame(maxWidth: .infinity)
    }
}

// MARK: - Current Exercise View

struct CurrentExerciseView: View {
    @EnvironmentObject var workoutManager: WorkoutManager
    let exercise: WorkoutExerciseUI
    let exerciseNumber: Int
    let totalExercises: Int

    @State private var weight: String = ""
    @State private var reps: String = ""

    var body: some View {
        VStack(spacing: 16) {
            // Exercise header
            HStack {
                Button(action: { workoutManager.previousExercise() }) {
                    Image(systemName: "chevron.left")
                        .font(.title2)
                        .foregroundColor(exerciseNumber > 1 ? .white : .gray)
                        .frame(width: 44, height: 44) // 44pt touch target
                }
                .disabled(exerciseNumber <= 1)

                Spacer()

                VStack(spacing: 4) {
                    Text(exercise.exercise.name)
                        .font(.title2.bold())
                        .foregroundColor(.white)
                    Text("\(exerciseNumber) of \(totalExercises)")
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()

                Button(action: { workoutManager.nextExercise() }) {
                    Image(systemName: "chevron.right")
                        .font(.title2)
                        .foregroundColor(exerciseNumber < totalExercises ? .white : .gray)
                        .frame(width: 44, height: 44) // 44pt touch target
                }
                .disabled(exerciseNumber >= totalExercises)
            }
            .padding(.horizontal)

            // Previous sets
            if !exercise.sets.isEmpty {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 12) {
                        ForEach(exercise.sets) { set in
                            CompletedSetBadge(set: set)
                        }
                    }
                    .padding(.horizontal)
                }
                .frame(height: 60)
            }

            // Set input - Large touch targets
            VStack(spacing: 20) {
                HStack(spacing: 24) {
                    // Weight input
                    VStack(spacing: 8) {
                        Text("Weight")
                            .font(.caption)
                            .foregroundColor(.gray)
                        LargeNumberInput(
                            value: $weight,
                            placeholder: lastWeight,
                            unit: "lbs"
                        )
                    }

                    // Reps input
                    VStack(spacing: 8) {
                        Text("Reps")
                            .font(.caption)
                            .foregroundColor(.gray)
                        LargeNumberInput(
                            value: $reps,
                            placeholder: lastReps,
                            unit: ""
                        )
                    }
                }

                // Log buttons - Large touch targets (minimum 44pt)
                HStack(spacing: 16) {
                    // Quick log button
                    if !exercise.sets.isEmpty {
                        Button(action: { workoutManager.quickLog() }) {
                            HStack {
                                Image(systemName: "repeat")
                                Text("Same")
                            }
                            .font(.headline)
                            .foregroundColor(.white)
                            .frame(height: 56) // Larger than 44pt minimum
                            .frame(maxWidth: .infinity)
                            .background(Color.gray.opacity(0.3))
                            .cornerRadius(12)
                        }
                    }

                    // Log set button
                    Button(action: { logCurrentSet() }) {
                        HStack {
                            Image(systemName: "checkmark")
                            Text("Log Set")
                        }
                        .font(.headline.bold())
                        .foregroundColor(.black)
                        .frame(height: 56) // Larger than 44pt minimum
                        .frame(maxWidth: .infinity)
                        .background(Color.green)
                        .cornerRadius(12)
                    }
                }
                .padding(.horizontal)
            }
            .padding()
        }
    }

    private var lastWeight: String {
        if let last = exercise.sets.last {
            return String(format: "%.0f", last.weight)
        }
        return "135"
    }

    private var lastReps: String {
        if let last = exercise.sets.last {
            return "\(last.reps)"
        }
        return "10"
    }

    private func logCurrentSet() {
        let weightValue = Double(weight) ?? Double(lastWeight) ?? 0
        let repsValue = Int(reps) ?? Int(lastReps) ?? 0

        if weightValue > 0 && repsValue > 0 {
            workoutManager.logSet(weight: weightValue, reps: repsValue)
            weight = ""
            reps = ""
        }
    }
}

struct CompletedSetBadge: View {
    let set: WorkoutSetUI

    var body: some View {
        VStack(spacing: 2) {
            Text("Set \(set.setNumber)")
                .font(.caption2)
                .foregroundColor(.gray)
            Text("\(Int(set.weight))×\(set.reps)")
                .font(.subheadline.bold())
                .foregroundColor(.white)
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 8)
        .background(Color.green.opacity(0.2))
        .cornerRadius(8)
    }
}

struct LargeNumberInput: View {
    @Binding var value: String
    let placeholder: String
    let unit: String

    var body: some View {
        HStack(spacing: 4) {
            TextField(placeholder, text: $value)
                .keyboardType(.decimalPad)
                .font(.system(size: 32, weight: .bold, design: .rounded))
                .foregroundColor(.white)
                .multilineTextAlignment(.center)
                .frame(width: 100)

            if !unit.isEmpty {
                Text(unit)
                    .font(.subheadline)
                    .foregroundColor(.gray)
            }
        }
        .padding()
        .background(Color.gray.opacity(0.2))
        .cornerRadius(12)
    }
}

struct EmptyExerciseView: View {
    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "plus.circle.fill")
                .font(.system(size: 60))
                .foregroundColor(.green)

            Text("Add an exercise to get started")
                .font(.headline)
                .foregroundColor(.gray)

            Button(action: {}) {
                Text("Browse Exercises")
                    .font(.headline)
                    .foregroundColor(.black)
                    .padding()
                    .frame(height: 50)
                    .background(Color.green)
                    .cornerRadius(12)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

// MARK: - Rest Timer View

struct RestTimerView: View {
    let timeRemaining: Int
    let onSkip: () -> Void
    let onAddTime: () -> Void

    var body: some View {
        VStack(spacing: 20) {
            Text("REST")
                .font(.caption)
                .foregroundColor(.gray)

            Text(formatTime(timeRemaining))
                .font(.system(size: 64, weight: .bold, design: .monospaced))
                .foregroundColor(.white)

            HStack(spacing: 20) {
                // Add time button - 44pt touch target
                Button(action: onAddTime) {
                    Text("+30s")
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(width: 80, height: 44)
                        .background(Color.gray.opacity(0.3))
                        .cornerRadius(22)
                }

                // Skip button - 44pt touch target
                Button(action: onSkip) {
                    Text("Skip")
                        .font(.headline)
                        .foregroundColor(.black)
                        .frame(width: 80, height: 44)
                        .background(Color.green)
                        .cornerRadius(22)
                }
            }
        }
        .padding(32)
        .background(Color.black.opacity(0.9))
        .cornerRadius(20)
        .padding()
    }

    private func formatTime(_ seconds: Int) -> String {
        let minutes = seconds / 60
        let secs = seconds % 60
        return String(format: "%d:%02d", minutes, secs)
    }
}

// MARK: - Workout Control Bar

struct WorkoutControlBar: View {
    let isPaused: Bool
    let isResting: Bool
    let onPause: () -> Void
    let onResume: () -> Void
    let onEnd: () -> Void

    var body: some View {
        HStack(spacing: 24) {
            // End workout button - Large touch target
            Button(action: onEnd) {
                Image(systemName: "stop.fill")
                    .font(.title2)
                    .foregroundColor(.white)
                    .frame(width: 60, height: 60) // Well above 44pt minimum
                    .background(Color.red.opacity(0.8))
                    .clipShape(Circle())
            }

            Spacer()

            // Pause/Resume button - Large touch target
            Button(action: { isPaused ? onResume() : onPause() }) {
                Image(systemName: isPaused ? "play.fill" : "pause.fill")
                    .font(.title)
                    .foregroundColor(.black)
                    .frame(width: 80, height: 80) // Extra large for primary action
                    .background(Color.green)
                    .clipShape(Circle())
            }

            Spacer()

            // Placeholder for symmetry (could be settings/more options)
            Button(action: {}) {
                Image(systemName: "ellipsis")
                    .font(.title2)
                    .foregroundColor(.white)
                    .frame(width: 60, height: 60)
                    .background(Color.gray.opacity(0.3))
                    .clipShape(Circle())
            }
        }
        .padding(.horizontal, 32)
        .padding(.vertical, 20)
        .background(Color.black.opacity(0.5))
    }
}

#Preview {
    let manager = WorkoutManager()

    return ActiveWorkoutView()
        .environmentObject(manager)
        .onAppear {
            manager.startWorkout(
                name: "Push Day",
                exercises: [
                    ExerciseUI(id: UUID(), name: "Bench Press", muscleGroup: "Chest", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Overhead Press", muscleGroup: "Shoulders", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Tricep Pushdown", muscleGroup: "Triceps", exerciseType: "Resistance")
                ]
            )
        }
}
