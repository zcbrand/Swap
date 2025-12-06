// HomeView.swift
// Main home screen with quick workout start

import SwiftUI

/// Home view optimized for fast workout start.
/// HIG Pitfalls avoided:
/// - No forced account creation before exploration
/// - Minimal taps to start workout (core flow in 3 taps or fewer)
struct HomeView: View {
    @EnvironmentObject var workoutManager: WorkoutManager
    @EnvironmentObject var healthManager: HealthManager

    @State private var showWorkoutTemplates = false
    @State private var showQuickStart = false

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // Quick start section - primary action
                    QuickStartSection(
                        onQuickStart: { showQuickStart = true },
                        onTemplates: { showWorkoutTemplates = true }
                    )

                    // Activity rings preview (if authorized)
                    if healthManager.isAuthorized {
                        ActivityRingsPreview(rings: healthManager.activityRings)
                    }

                    // Recent workouts
                    RecentWorkoutsSection()

                    // Stats summary
                    WeeklyStatsSection()
                }
                .padding()
            }
            .navigationTitle("Swap")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: {}) {
                        Image(systemName: "gearshape")
                    }
                }
            }
        }
        .sheet(isPresented: $showQuickStart) {
            QuickStartSheet(onStart: startEmptyWorkout)
        }
        .sheet(isPresented: $showWorkoutTemplates) {
            WorkoutTemplatesSheet(onSelect: startFromTemplate)
        }
    }

    private func startEmptyWorkout(name: String) {
        showQuickStart = false
        workoutManager.startWorkout(name: name)
    }

    private func startFromTemplate(_ template: WorkoutTemplateUI) {
        showWorkoutTemplates = false
        workoutManager.startWorkout(
            name: template.name,
            exercises: template.exercises
        )
    }
}

// MARK: - Quick Start Section

struct QuickStartSection: View {
    let onQuickStart: () -> Void
    let onTemplates: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            // Primary quick start button - large touch target
            Button(action: onQuickStart) {
                HStack {
                    Image(systemName: "play.fill")
                        .font(.title2)
                    Text("Start Workout")
                        .font(.title3.bold())
                }
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .frame(height: 64) // Large touch target
                .background(
                    LinearGradient(
                        colors: [.green, .green.opacity(0.8)],
                        startPoint: .leading,
                        endPoint: .trailing
                    )
                )
                .cornerRadius(16)
            }

            // Templates button
            Button(action: onTemplates) {
                HStack {
                    Image(systemName: "doc.text")
                    Text("From Template")
                    Spacer()
                    Image(systemName: "chevron.right")
                        .foregroundColor(.secondary)
                }
                .foregroundColor(.primary)
                .padding()
                .frame(height: 56)
                .background(Color.gray.opacity(0.1))
                .cornerRadius(12)
            }
        }
    }
}

// MARK: - Activity Rings Preview

struct ActivityRingsPreview: View {
    let rings: ActivityRingsData

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Today's Activity")
                    .font(.headline)
                Spacer()
                NavigationLink(destination: Text("Activity Details")) {
                    Text("See All")
                        .font(.subheadline)
                        .foregroundColor(.green)
                }
            }

            HStack(spacing: 16) {
                // Simplified ring display
                ActivityRingView(
                    progress: rings.moveProgress,
                    color: Color(hex: ActivityRingsData.moveColor) ?? .red,
                    label: "Move",
                    value: "\(Int(rings.moveCalories))",
                    unit: "CAL"
                )

                ActivityRingView(
                    progress: rings.exerciseProgress,
                    color: Color(hex: ActivityRingsData.exerciseColor) ?? .green,
                    label: "Exercise",
                    value: "\(Int(rings.exerciseMinutes))",
                    unit: "MIN"
                )

                ActivityRingView(
                    progress: rings.standProgress,
                    color: Color(hex: ActivityRingsData.standColor) ?? .cyan,
                    label: "Stand",
                    value: "\(Int(rings.standHours))",
                    unit: "HRS"
                )
            }
        }
        .padding()
        .background(Color.gray.opacity(0.1))
        .cornerRadius(16)
    }
}

struct ActivityRingView: View {
    let progress: Double
    let color: Color
    let label: String
    let value: String
    let unit: String

    var body: some View {
        VStack(spacing: 8) {
            ZStack {
                Circle()
                    .stroke(color.opacity(0.2), lineWidth: 8)
                Circle()
                    .trim(from: 0, to: min(progress, 1.0))
                    .stroke(color, style: StrokeStyle(lineWidth: 8, lineCap: .round))
                    .rotationEffect(.degrees(-90))

                VStack(spacing: 0) {
                    Text(value)
                        .font(.system(.body, design: .rounded).bold())
                    Text(unit)
                        .font(.system(size: 8))
                        .foregroundColor(.secondary)
                }
            }
            .frame(width: 60, height: 60)

            Text(label)
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
    }
}

// MARK: - Recent Workouts Section

struct RecentWorkoutsSection: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Recent Workouts")
                    .font(.headline)
                Spacer()
                NavigationLink(destination: Text("All Workouts")) {
                    Text("See All")
                        .font(.subheadline)
                        .foregroundColor(.green)
                }
            }

            // Placeholder for recent workouts
            VStack(spacing: 8) {
                RecentWorkoutRow(
                    name: "Push Day",
                    duration: "45 min",
                    date: "Today"
                )
                RecentWorkoutRow(
                    name: "Pull Day",
                    duration: "52 min",
                    date: "Yesterday"
                )
            }
        }
    }
}

struct RecentWorkoutRow: View {
    let name: String
    let duration: String
    let date: String

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(name)
                    .font(.subheadline.bold())
                Text("\(duration) • \(date)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            Spacer()
            Image(systemName: "chevron.right")
                .foregroundColor(.secondary)
        }
        .padding()
        .background(Color.gray.opacity(0.1))
        .cornerRadius(12)
    }
}

// MARK: - Weekly Stats Section

struct WeeklyStatsSection: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("This Week")
                .font(.headline)

            HStack(spacing: 12) {
                WeeklyStatCard(value: "4", label: "Workouts", icon: "dumbbell.fill")
                WeeklyStatCard(value: "3.2h", label: "Total Time", icon: "clock.fill")
                WeeklyStatCard(value: "12.4k", label: "Volume (lbs)", icon: "scalemass.fill")
            }
        }
    }
}

struct WeeklyStatCard: View {
    let value: String
    let label: String
    let icon: String

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .foregroundColor(.green)
            Text(value)
                .font(.headline)
            Text(label)
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 16)
        .background(Color.gray.opacity(0.1))
        .cornerRadius(12)
    }
}

// MARK: - Quick Start Sheet

struct QuickStartSheet: View {
    let onStart: (String) -> Void
    @State private var workoutName = ""
    @Environment(\.dismiss) var dismiss

    var body: some View {
        NavigationView {
            VStack(spacing: 24) {
                TextField("Workout Name", text: $workoutName)
                    .textFieldStyle(.roundedBorder)
                    .font(.title3)
                    .padding()

                Text("Or choose a quick option:")
                    .font(.subheadline)
                    .foregroundColor(.secondary)

                VStack(spacing: 12) {
                    QuickOptionButton(title: "Push Day", icon: "arrow.up") {
                        onStart("Push Day")
                    }
                    QuickOptionButton(title: "Pull Day", icon: "arrow.down") {
                        onStart("Pull Day")
                    }
                    QuickOptionButton(title: "Leg Day", icon: "figure.walk") {
                        onStart("Leg Day")
                    }
                    QuickOptionButton(title: "Full Body", icon: "figure.mixed.cardio") {
                        onStart("Full Body")
                    }
                }
                .padding(.horizontal)

                Spacer()
            }
            .navigationTitle("Start Workout")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Start") {
                        onStart(workoutName.isEmpty ? "Workout" : workoutName)
                    }
                    .bold()
                }
            }
        }
    }
}

struct QuickOptionButton: View {
    let title: String
    let icon: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: icon)
                Text(title)
                Spacer()
                Image(systemName: "chevron.right")
                    .foregroundColor(.secondary)
            }
            .padding()
            .frame(height: 56) // Large touch target
            .background(Color.gray.opacity(0.1))
            .cornerRadius(12)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Workout Templates Sheet

struct WorkoutTemplatesSheet: View {
    let onSelect: (WorkoutTemplateUI) -> Void
    @Environment(\.dismiss) var dismiss

    var body: some View {
        NavigationView {
            List {
                Section("My Templates") {
                    Text("No custom templates yet")
                        .foregroundColor(.secondary)
                }

                Section("Suggested") {
                    ForEach(sampleTemplates) { template in
                        Button(action: { onSelect(template) }) {
                            HStack {
                                VStack(alignment: .leading) {
                                    Text(template.name)
                                        .font(.headline)
                                    Text("\(template.exercises.count) exercises • ~\(template.estimatedMinutes) min")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                                Spacer()
                                Image(systemName: "chevron.right")
                                    .foregroundColor(.secondary)
                            }
                        }
                        .buttonStyle(.plain)
                    }
                }
            }
            .navigationTitle("Templates")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
        }
    }

    private var sampleTemplates: [WorkoutTemplateUI] {
        [
            WorkoutTemplateUI(
                id: UUID(),
                name: "Push Day",
                exercises: [
                    ExerciseUI(id: UUID(), name: "Bench Press", muscleGroup: "Chest", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Incline Dumbbell Press", muscleGroup: "Chest", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Overhead Press", muscleGroup: "Shoulders", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Lateral Raises", muscleGroup: "Shoulders", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Tricep Pushdown", muscleGroup: "Triceps", exerciseType: "Resistance"),
                ],
                estimatedMinutes: 45
            ),
            WorkoutTemplateUI(
                id: UUID(),
                name: "Pull Day",
                exercises: [
                    ExerciseUI(id: UUID(), name: "Deadlift", muscleGroup: "Back", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Barbell Row", muscleGroup: "Back", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Lat Pulldown", muscleGroup: "Back", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Face Pulls", muscleGroup: "Shoulders", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Barbell Curl", muscleGroup: "Biceps", exerciseType: "Resistance"),
                ],
                estimatedMinutes: 50
            ),
            WorkoutTemplateUI(
                id: UUID(),
                name: "Leg Day",
                exercises: [
                    ExerciseUI(id: UUID(), name: "Squat", muscleGroup: "Quadriceps", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Romanian Deadlift", muscleGroup: "Hamstrings", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Leg Press", muscleGroup: "Quadriceps", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Leg Curl", muscleGroup: "Hamstrings", exerciseType: "Resistance"),
                    ExerciseUI(id: UUID(), name: "Calf Raises", muscleGroup: "Calves", exerciseType: "Resistance"),
                ],
                estimatedMinutes: 55
            ),
        ]
    }
}

// MARK: - Supporting Types

struct WorkoutTemplateUI: Identifiable {
    let id: UUID
    let name: String
    let exercises: [ExerciseUI]
    let estimatedMinutes: Int
}

// MARK: - Color Extension

extension Color {
    init?(hex: String) {
        var hexSanitized = hex.trimmingCharacters(in: .whitespacesAndNewlines)
        hexSanitized = hexSanitized.replacingOccurrences(of: "#", with: "")

        var rgb: UInt64 = 0

        guard Scanner(string: hexSanitized).scanHexInt64(&rgb) else { return nil }

        self.init(
            red: Double((rgb & 0xFF0000) >> 16) / 255.0,
            green: Double((rgb & 0x00FF00) >> 8) / 255.0,
            blue: Double(rgb & 0x0000FF) / 255.0
        )
    }
}

#Preview {
    HomeView()
        .environmentObject(WorkoutManager())
        .environmentObject(HealthManager())
}
