// SwapWidget.swift
// iOS Widget implementation following HIG guidelines

import WidgetKit
import SwiftUI

/// Widget entry containing data to display.
struct SwapWidgetEntry: TimelineEntry {
    let date: Date
    let streak: Int
    let workoutsThisWeek: Int
    let lastWorkoutName: String?
    let lastWorkoutDate: Date?
    let calories: Int
}

/// Provider that supplies timeline entries to the widget.
struct SwapWidgetProvider: TimelineProvider {
    func placeholder(in context: Context) -> SwapWidgetEntry {
        SwapWidgetEntry(
            date: Date(),
            streak: 5,
            workoutsThisWeek: 3,
            lastWorkoutName: "Push Day",
            lastWorkoutDate: Date(),
            calories: 450
        )
    }

    func getSnapshot(in context: Context, completion: @escaping (SwapWidgetEntry) -> Void) {
        let entry = SwapWidgetEntry(
            date: Date(),
            streak: 5,
            workoutsThisWeek: 3,
            lastWorkoutName: "Push Day",
            lastWorkoutDate: Date(),
            calories: 450
        )
        completion(entry)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<SwapWidgetEntry>) -> Void) {
        // In production, this would fetch real data from shared storage
        let entry = SwapWidgetEntry(
            date: Date(),
            streak: 5,
            workoutsThisWeek: 3,
            lastWorkoutName: "Push Day",
            lastWorkoutDate: Date().addingTimeInterval(-3600),
            calories: 450
        )

        // Refresh at start of next day
        let calendar = Calendar.current
        let tomorrow = calendar.startOfDay(for: Date().addingTimeInterval(86400))
        let timeline = Timeline(entries: [entry], policy: .after(tomorrow))

        completion(timeline)
    }
}

// MARK: - Small Widget View

/// Small widget: Single metric focus (streak)
/// HIG: Glanceable, most significant metric
struct SmallWidgetView: View {
    let entry: SwapWidgetEntry

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: "flame.fill")
                .font(.title)
                .foregroundColor(.orange)

            Text("\(entry.streak)")
                .font(.system(size: 48, weight: .bold, design: .rounded))

            Text("Day Streak")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(.systemBackground))
    }
}

// MARK: - Medium Widget View

/// Medium widget: 2-3 metrics
/// HIG: Appropriate information density for size
struct MediumWidgetView: View {
    let entry: SwapWidgetEntry

    var body: some View {
        HStack(spacing: 16) {
            // Streak
            VStack(spacing: 4) {
                Image(systemName: "flame.fill")
                    .foregroundColor(.orange)
                Text("\(entry.streak)")
                    .font(.system(.title, design: .rounded).bold())
                Text("Streak")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
            .frame(maxWidth: .infinity)

            Divider()

            // This week
            VStack(spacing: 4) {
                Image(systemName: "dumbbell.fill")
                    .foregroundColor(.green)
                Text("\(entry.workoutsThisWeek)")
                    .font(.system(.title, design: .rounded).bold())
                Text("This Week")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
            .frame(maxWidth: .infinity)

            Divider()

            // Calories
            VStack(spacing: 4) {
                Image(systemName: "bolt.fill")
                    .foregroundColor(.yellow)
                Text("\(entry.calories)")
                    .font(.system(.title, design: .rounded).bold())
                Text("Calories")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
            .frame(maxWidth: .infinity)
        }
        .padding()
        .background(Color(.systemBackground))
    }
}

// MARK: - Large Widget View

/// Large widget: Full summary with quick actions
/// HIG: Appropriate information density, actionable
struct LargeWidgetView: View {
    let entry: SwapWidgetEntry

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            // Header
            HStack {
                Image(systemName: "flame.fill")
                    .foregroundColor(.orange)
                Text("\(entry.streak) Day Streak")
                    .font(.headline)
                Spacer()
            }

            Divider()

            // Stats grid
            HStack(spacing: 20) {
                StatBlock(value: "\(entry.workoutsThisWeek)", label: "Workouts", sublabel: "this week")
                StatBlock(value: "\(entry.calories)", label: "Calories", sublabel: "burned")
            }

            Divider()

            // Last workout
            if let lastWorkout = entry.lastWorkoutName {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Last Workout")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text(lastWorkout)
                        .font(.headline)
                    if let date = entry.lastWorkoutDate {
                        Text(formatRelativeDate(date))
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }

            Spacer()

            // Quick start hint
            HStack {
                Image(systemName: "play.circle.fill")
                    .foregroundColor(.green)
                Text("Tap to start workout")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
        .background(Color(.systemBackground))
    }

    private func formatRelativeDate(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .short
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

struct StatBlock: View {
    let value: String
    let label: String
    let sublabel: String

    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            Text(value)
                .font(.system(.title2, design: .rounded).bold())
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)
            Text(sublabel)
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}

// MARK: - Widget Configuration

struct SwapWidget: Widget {
    let kind: String = "SwapWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: SwapWidgetProvider()) { entry in
            SwapWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Swap Workout")
        .description("Track your workout streak and weekly progress.")
        .supportedFamilies([.systemSmall, .systemMedium, .systemLarge])
    }
}

struct SwapWidgetEntryView: View {
    @Environment(\.widgetFamily) var family
    let entry: SwapWidgetEntry

    var body: some View {
        switch family {
        case .systemSmall:
            SmallWidgetView(entry: entry)
        case .systemMedium:
            MediumWidgetView(entry: entry)
        case .systemLarge:
            LargeWidgetView(entry: entry)
        default:
            SmallWidgetView(entry: entry)
        }
    }
}

// MARK: - Quick Start Widget

/// Quick Start Widget for one-tap workout start
struct QuickStartWidgetProvider: TimelineProvider {
    func placeholder(in context: Context) -> QuickStartEntry {
        QuickStartEntry(date: Date(), hasActiveWorkout: false)
    }

    func getSnapshot(in context: Context, completion: @escaping (QuickStartEntry) -> Void) {
        completion(QuickStartEntry(date: Date(), hasActiveWorkout: false))
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<QuickStartEntry>) -> Void) {
        let entry = QuickStartEntry(date: Date(), hasActiveWorkout: false)
        let timeline = Timeline(entries: [entry], policy: .after(Date().addingTimeInterval(3600)))
        completion(timeline)
    }
}

struct QuickStartEntry: TimelineEntry {
    let date: Date
    let hasActiveWorkout: Bool
}

struct QuickStartWidget: Widget {
    let kind = "QuickStartWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: QuickStartWidgetProvider()) { entry in
            QuickStartWidgetView(entry: entry)
        }
        .configurationDisplayName("Quick Start")
        .description("Start a workout with one tap.")
        .supportedFamilies([.systemSmall])
    }
}

struct QuickStartWidgetView: View {
    let entry: QuickStartEntry

    var body: some View {
        VStack(spacing: 12) {
            ZStack {
                Circle()
                    .fill(Color.green)
                    .frame(width: 60, height: 60)
                Image(systemName: entry.hasActiveWorkout ? "pause.fill" : "play.fill")
                    .font(.title)
                    .foregroundColor(.white)
            }

            Text(entry.hasActiveWorkout ? "Resume" : "Start")
                .font(.headline)

            Text("Workout")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(.systemBackground))
        .widgetURL(URL(string: "swap://start")!)
    }
}

// MARK: - Widget Bundle

@main
struct SwapWidgets: WidgetBundle {
    var body: some Widget {
        SwapWidget()
        QuickStartWidget()
    }
}

// MARK: - Previews

struct SwapWidget_Previews: PreviewProvider {
    static var previews: some View {
        let entry = SwapWidgetEntry(
            date: Date(),
            streak: 5,
            workoutsThisWeek: 3,
            lastWorkoutName: "Push Day",
            lastWorkoutDate: Date().addingTimeInterval(-3600),
            calories: 450
        )

        Group {
            SmallWidgetView(entry: entry)
                .previewContext(WidgetPreviewContext(family: .systemSmall))
                .previewDisplayName("Small")

            MediumWidgetView(entry: entry)
                .previewContext(WidgetPreviewContext(family: .systemMedium))
                .previewDisplayName("Medium")

            LargeWidgetView(entry: entry)
                .previewContext(WidgetPreviewContext(family: .systemLarge))
                .previewDisplayName("Large")
        }
    }
}
