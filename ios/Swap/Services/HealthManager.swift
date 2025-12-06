// HealthManager.swift
// HealthKit integration following HIG requirements

import Foundation
import HealthKit
import Combine

/// Manages HealthKit integration.
/// HIG Requirements:
/// - Request access only when needed (not at launch)
/// - Use "the Health app" terminology in user-facing text
@MainActor
class HealthManager: ObservableObject {
    // MARK: - Published Properties

    @Published var isAuthorized = false
    @Published var authorizationStatus: AuthorizationStatus = .notDetermined
    @Published var activityRings: ActivityRingsData = .empty
    @Published var isSyncing = false
    @Published var lastSyncError: String?

    // MARK: - Private Properties

    private let healthStore = HKHealthStore()

    // Types we read from the Health app
    private let readTypes: Set<HKObjectType> = {
        var types: Set<HKObjectType> = []
        if let activeEnergy = HKQuantityType.quantityType(forIdentifier: .activeEnergyBurned) {
            types.insert(activeEnergy)
        }
        if let exerciseTime = HKQuantityType.quantityType(forIdentifier: .appleExerciseTime) {
            types.insert(exerciseTime)
        }
        if let standHours = HKCategoryType.categoryType(forIdentifier: .appleStandHour) {
            types.insert(standHours)
        }
        if let workoutType = HKObjectType.workoutType() as HKObjectType? {
            types.insert(workoutType)
        }
        return types
    }()

    // Types we write to the Health app
    private let writeTypes: Set<HKSampleType> = {
        var types: Set<HKSampleType> = []
        if let activeEnergy = HKQuantityType.quantityType(forIdentifier: .activeEnergyBurned) {
            types.insert(activeEnergy)
        }
        types.insert(HKObjectType.workoutType())
        return types
    }()

    // MARK: - Authorization

    /// Check if HealthKit is available on this device.
    var isHealthDataAvailable: Bool {
        HKHealthStore.isHealthDataAvailable()
    }

    /// Request authorization to access the Health app.
    /// Only call this when the user initiates an action that requires health data.
    func requestAuthorization() async throws {
        guard isHealthDataAvailable else {
            authorizationStatus = .unavailable
            throw HealthError.unavailable
        }

        try await healthStore.requestAuthorization(toShare: writeTypes, read: readTypes)

        // Check the authorization status
        await updateAuthorizationStatus()
    }

    private func updateAuthorizationStatus() async {
        guard isHealthDataAvailable else {
            authorizationStatus = .unavailable
            return
        }

        // Check if we can write workouts
        let workoutStatus = healthStore.authorizationStatus(for: HKObjectType.workoutType())

        switch workoutStatus {
        case .sharingAuthorized:
            authorizationStatus = .authorized
            isAuthorized = true
        case .sharingDenied:
            authorizationStatus = .denied
            isAuthorized = false
        case .notDetermined:
            authorizationStatus = .notDetermined
            isAuthorized = false
        @unknown default:
            authorizationStatus = .notDetermined
            isAuthorized = false
        }
    }

    // MARK: - Activity Rings

    /// Fetch current Activity ring data.
    func fetchActivityRings() async throws -> ActivityRingsData {
        guard isAuthorized else {
            throw HealthError.notAuthorized
        }

        let calendar = Calendar.current
        let now = Date()
        let startOfDay = calendar.startOfDay(for: now)

        async let moveCalories = fetchActiveEnergy(from: startOfDay, to: now)
        async let exerciseMinutes = fetchExerciseMinutes(from: startOfDay, to: now)
        async let standHours = fetchStandHours(from: startOfDay, to: now)

        let (move, exercise, stand) = try await (moveCalories, exerciseMinutes, standHours)

        // Default goals (these would come from user settings in production)
        let moveGoal: Double = 500
        let exerciseGoal: Double = 30
        let standGoal: Double = 12

        let rings = ActivityRingsData(
            moveCalories: move,
            moveGoal: moveGoal,
            moveProgress: move / moveGoal,
            exerciseMinutes: exercise,
            exerciseGoal: exerciseGoal,
            exerciseProgress: exercise / exerciseGoal,
            standHours: stand,
            standGoal: standGoal,
            standProgress: stand / standGoal
        )

        await MainActor.run {
            self.activityRings = rings
        }

        return rings
    }

    private func fetchActiveEnergy(from startDate: Date, to endDate: Date) async throws -> Double {
        guard let activeEnergyType = HKQuantityType.quantityType(forIdentifier: .activeEnergyBurned) else {
            return 0
        }

        let predicate = HKQuery.predicateForSamples(withStart: startDate, end: endDate)
        let options: HKStatisticsOptions = .cumulativeSum

        return try await withCheckedThrowingContinuation { continuation in
            let query = HKStatisticsQuery(
                quantityType: activeEnergyType,
                quantitySamplePredicate: predicate,
                options: options
            ) { _, result, error in
                if let error = error {
                    continuation.resume(throwing: error)
                    return
                }

                let calories = result?.sumQuantity()?.doubleValue(for: .kilocalorie()) ?? 0
                continuation.resume(returning: calories)
            }

            healthStore.execute(query)
        }
    }

    private func fetchExerciseMinutes(from startDate: Date, to endDate: Date) async throws -> Double {
        guard let exerciseType = HKQuantityType.quantityType(forIdentifier: .appleExerciseTime) else {
            return 0
        }

        let predicate = HKQuery.predicateForSamples(withStart: startDate, end: endDate)

        return try await withCheckedThrowingContinuation { continuation in
            let query = HKStatisticsQuery(
                quantityType: exerciseType,
                quantitySamplePredicate: predicate,
                options: .cumulativeSum
            ) { _, result, error in
                if let error = error {
                    continuation.resume(throwing: error)
                    return
                }

                let minutes = result?.sumQuantity()?.doubleValue(for: .minute()) ?? 0
                continuation.resume(returning: minutes)
            }

            healthStore.execute(query)
        }
    }

    private func fetchStandHours(from startDate: Date, to endDate: Date) async throws -> Double {
        guard let standType = HKCategoryType.categoryType(forIdentifier: .appleStandHour) else {
            return 0
        }

        let predicate = HKQuery.predicateForSamples(withStart: startDate, end: endDate)

        return try await withCheckedThrowingContinuation { continuation in
            let query = HKSampleQuery(
                sampleType: standType,
                predicate: predicate,
                limit: HKObjectQueryNoLimit,
                sortDescriptors: nil
            ) { _, samples, error in
                if let error = error {
                    continuation.resume(throwing: error)
                    return
                }

                // Count hours where user stood
                let standHours = samples?.filter { sample in
                    (sample as? HKCategorySample)?.value == HKCategoryValueAppleStandHour.stood.rawValue
                }.count ?? 0

                continuation.resume(returning: Double(standHours))
            }

            healthStore.execute(query)
        }
    }

    // MARK: - Workout Syncing

    /// Save a completed workout to the Health app.
    func saveWorkout(_ summary: WorkoutSummaryUI) async throws {
        guard isAuthorized else {
            throw HealthError.notAuthorized
        }

        isSyncing = true
        lastSyncError = nil

        defer {
            Task { @MainActor in
                self.isSyncing = false
            }
        }

        let startDate = summary.completedAt.addingTimeInterval(-summary.duration)
        let endDate = summary.completedAt

        // Create the workout
        let workout = HKWorkout(
            activityType: .traditionalStrengthTraining,
            start: startDate,
            end: endDate,
            workoutEvents: nil,
            totalEnergyBurned: HKQuantity(unit: .kilocalorie(), doubleValue: Double(summary.caloriesBurned)),
            totalDistance: nil,
            metadata: [
                HKMetadataKeyWorkoutBrandName: "Swap",
                "WorkoutName": summary.name,
                "TotalSets": summary.totalSets,
                "TotalVolume": summary.totalVolume
            ]
        )

        try await healthStore.save(workout)

        // Also save the active energy burned
        if let activeEnergyType = HKQuantityType.quantityType(forIdentifier: .activeEnergyBurned) {
            let energySample = HKQuantitySample(
                type: activeEnergyType,
                quantity: HKQuantity(unit: .kilocalorie(), doubleValue: Double(summary.caloriesBurned)),
                start: startDate,
                end: endDate,
                metadata: ["SwapWorkoutID": summary.id.uuidString]
            )

            try await healthStore.save(energySample)
        }
    }

    /// Get a user-friendly message for requesting Health access.
    /// Uses "the Health app" terminology as per HIG.
    static var authorizationMessage: String {
        "Swap can sync your workouts with the Health app to contribute to your Activity rings and track your fitness progress."
    }
}

// MARK: - Supporting Types

enum AuthorizationStatus {
    case notDetermined
    case authorized
    case denied
    case unavailable
}

enum HealthError: LocalizedError {
    case unavailable
    case notAuthorized
    case syncFailed(String)

    var errorDescription: String? {
        switch self {
        case .unavailable:
            return "Health data is not available on this device."
        case .notAuthorized:
            return "Please grant access to the Health app in Settings."
        case .syncFailed(let reason):
            return "Failed to sync with the Health app: \(reason)"
        }
    }
}

/// Activity rings data with Apple's exact colors per HIG.
struct ActivityRingsData {
    let moveCalories: Double
    let moveGoal: Double
    let moveProgress: Double

    let exerciseMinutes: Double
    let exerciseGoal: Double
    let exerciseProgress: Double

    let standHours: Double
    let standGoal: Double
    let standProgress: Double

    // HIG-compliant Activity ring colors
    static let moveColor = "#FA114F"     // Red
    static let exerciseColor = "#92E82A" // Green
    static let standColor = "#00D4FF"    // Blue

    static let empty = ActivityRingsData(
        moveCalories: 0,
        moveGoal: 500,
        moveProgress: 0,
        exerciseMinutes: 0,
        exerciseGoal: 30,
        exerciseProgress: 0,
        standHours: 0,
        standGoal: 12,
        standProgress: 0
    )
}
