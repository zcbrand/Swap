// SwapApp.swift
// Main app entry point

import SwiftUI

@main
struct SwapApp: App {
    @StateObject private var workoutManager = WorkoutManager()
    @StateObject private var healthManager = HealthManager()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(workoutManager)
                .environmentObject(healthManager)
        }
    }
}

struct ContentView: View {
    @EnvironmentObject var workoutManager: WorkoutManager

    var body: some View {
        ZStack {
            if workoutManager.isWorkoutActive {
                ActiveWorkoutView()
                    .transition(.move(edge: .bottom))
            } else {
                HomeView()
            }
        }
        .animation(.easeInOut(duration: 0.3), value: workoutManager.isWorkoutActive)
    }
}
