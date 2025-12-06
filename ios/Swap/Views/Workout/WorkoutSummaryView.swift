// WorkoutSummaryView.swift
// End-of-workout summary with Activity Rings integration option

import SwiftUI

/// Summary screen at workout end with option for Activity Rings integration.
/// HIG Requirements:
/// - Clear feedback when session stops
/// - Option to sync with the Health app
struct WorkoutSummaryView: View {
    let summary: WorkoutSummaryUI
    let onDismiss: () -> Void
    let onSyncToHealth: () -> Void

    @EnvironmentObject var healthManager: HealthManager
    @State private var isSyncing = false
    @State private var syncComplete = false
    @State private var syncError: String?

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // Celebration header
                    VStack(spacing: 12) {
                        Image(systemName: "checkmark.circle.fill")
                            .font(.system(size: 80))
                            .foregroundColor(.green)

                        Text("Workout Complete!")
                            .font(.largeTitle.bold())

                        Text(summary.name)
                            .font(.title3)
                            .foregroundColor(.secondary)
                    }
                    .padding(.top, 20)

                    // Main stats
                    HStack(spacing: 0) {
                        StatCard(
                            value: formatDuration(summary.duration),
                            label: "Duration",
                            icon: "clock.fill"
                        )

                        StatCard(
                            value: "\(summary.totalSets)",
                            label: "Sets",
                            icon: "checkmark.circle.fill"
                        )

                        StatCard(
                            value: formatVolume(summary.totalVolume),
                            label: "Volume",
                            icon: "scalemass.fill"
                        )
                    }
                    .padding(.horizontal)

                    // Calories
                    HStack {
                        Image(systemName: "flame.fill")
                            .foregroundColor(.orange)
                        Text("\(summary.caloriesBurned)")
                            .font(.title2.bold())
                        Text("calories burned")
                            .foregroundColor(.secondary)
                    }
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.orange.opacity(0.1))
                    .cornerRadius(12)
                    .padding(.horizontal)

                    // Health sync section
                    HealthSyncSection(
                        isAuthorized: healthManager.isAuthorized,
                        isSyncing: isSyncing,
                        syncComplete: syncComplete,
                        syncError: syncError,
                        onSync: syncToHealth,
                        onRequestAccess: requestHealthAccess
                    )
                    .padding(.horizontal)

                    Spacer(minLength: 40)

                    // Done button
                    Button(action: onDismiss) {
                        Text("Done")
                            .font(.headline)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .frame(height: 56)
                            .background(Color.green)
                            .cornerRadius(12)
                    }
                    .padding(.horizontal)
                    .padding(.bottom, 20)
                }
            }
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        onDismiss()
                    }
                }
            }
        }
    }

    private func formatDuration(_ duration: TimeInterval) -> String {
        let minutes = Int(duration) / 60
        if minutes >= 60 {
            let hours = minutes / 60
            let mins = minutes % 60
            return "\(hours)h \(mins)m"
        }
        return "\(minutes)m"
    }

    private func formatVolume(_ volume: Double) -> String {
        if volume >= 1000 {
            return String(format: "%.1fk", volume / 1000)
        }
        return String(format: "%.0f", volume)
    }

    private func syncToHealth() {
        isSyncing = true
        Task {
            do {
                try await healthManager.saveWorkout(summary)
                await MainActor.run {
                    syncComplete = true
                    isSyncing = false
                }
            } catch {
                await MainActor.run {
                    syncError = error.localizedDescription
                    isSyncing = false
                }
            }
        }
    }

    private func requestHealthAccess() {
        Task {
            do {
                try await healthManager.requestAuthorization()
            } catch {
                syncError = error.localizedDescription
            }
        }
    }
}

struct StatCard: View {
    let value: String
    let label: String
    let icon: String

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(.green)

            Text(value)
                .font(.title2.bold())

            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 16)
        .background(Color.gray.opacity(0.1))
        .cornerRadius(12)
    }
}

struct HealthSyncSection: View {
    let isAuthorized: Bool
    let isSyncing: Bool
    let syncComplete: Bool
    let syncError: String?
    let onSync: () -> Void
    let onRequestAccess: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            HStack {
                Image(systemName: "heart.fill")
                    .foregroundColor(.red)
                Text("Health App")
                    .font(.headline)
                Spacer()
            }

            if let error = syncError {
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            } else if syncComplete {
                HStack {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                    Text("Synced with the Health app")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
            } else if isAuthorized {
                Button(action: onSync) {
                    HStack {
                        if isSyncing {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            Image(systemName: "arrow.triangle.2.circlepath")
                        }
                        Text(isSyncing ? "Syncing..." : "Sync to Health")
                    }
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .frame(height: 50)
                    .background(Color.red.opacity(0.8))
                    .cornerRadius(10)
                }
                .disabled(isSyncing)
            } else {
                VStack(spacing: 12) {
                    Text(HealthManager.authorizationMessage)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)

                    Button(action: onRequestAccess) {
                        HStack {
                            Image(systemName: "heart.fill")
                            Text("Connect to Health")
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .frame(height: 50)
                        .background(Color.red.opacity(0.8))
                        .cornerRadius(10)
                    }
                }
            }
        }
        .padding()
        .background(Color.gray.opacity(0.1))
        .cornerRadius(12)
    }
}

#Preview {
    let summary = WorkoutSummaryUI(
        id: UUID(),
        name: "Push Day",
        duration: 3600,
        totalVolume: 5420,
        totalSets: 18,
        totalExercises: 5,
        caloriesBurned: 342,
        completedAt: Date()
    )

    return WorkoutSummaryView(
        summary: summary,
        onDismiss: {},
        onSyncToHealth: {}
    )
    .environmentObject(HealthManager())
}
