# Swap - Workout Tracking App

A cross-platform workout tracking app with a Rust backend and native mobile apps for iOS and Android.

## Architecture

```
swap/
├── crates/
│   ├── swap-core/     # Shared business logic (Rust → UniFFI → Swift/Kotlin)
│   └── swap-api/      # REST API backend (Axum)
├── ios/               # iOS app (SwiftUI)
├── android/           # Android app (Jetpack Compose) [planned]
└── bindings/          # Generated UniFFI bindings
```

## Features

### Core Functionality
- **Fast Set Logging**: Log sets in under 5 seconds with quick-log feature
- **1RM Calculations**: Multiple formulas (Epley, Brzycki, etc.) averaged for accuracy
- **Workout Sessions**: Full state machine with pause/resume/rest timer
- **Progress Tracking**: Metrics, streaks, and personal records

### iOS (HIG Compliant)
- **Large Touch Targets**: All controls minimum 44pt for workout mode
- **Distinct Workout Mode**: Dark theme with clear visual differentiation
- **Real-time Metrics**: Updates without user action
- **HealthKit Integration**: Syncs to Activity Rings with proper authorization flow
- **Widgets**: Small/Medium/Large with appropriate information density

### Backend
- RESTful API with Axum
- SQLite database with proper migrations
- Sync protocol for multi-device support
- Zero data loss design

## Getting Started

### Prerequisites
- Rust 1.75+
- Xcode 15+ (for iOS)
- Android Studio (for Android) [coming soon]

### Build the Rust Backend

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run the API server
cargo run -p swap-api
```

### iOS Development

1. Generate UniFFI bindings:
```bash
cargo build -p swap-core --release
# UniFFI bindings will be generated in bindings/swift/
```

2. Open `ios/Swap.xcodeproj` in Xcode

3. Build and run on simulator or device

## Design Principles

Following iOS Human Interface Guidelines and avoiding common pitfalls:

### Do
- ✅ Large, easily tapped controls (44pt minimum)
- ✅ Real-time metrics without user action
- ✅ Core logging flow in 3 taps or fewer
- ✅ Request Health access only when needed
- ✅ Use "the Health app" terminology
- ✅ Maintain Activity ring colors exactly as specified

### Avoid
- ❌ Multiple taps for basic logging
- ❌ Feature bloat overwhelming users
- ❌ Forced account creation before exploration
- ❌ Intrusive ads during workouts
- ❌ Small touch targets during exertion
- ❌ Data sync failures

## 1RM Calculation

We implement multiple formulas and average them for best accuracy:

- **Epley**: `1RM = Weight × (1 + Reps/30)`
- **Brzycki**: `1RM = Weight × (36/(37 - Reps))`
- **Lombardi**: `1RM = Weight × Reps^0.10`
- **Mayhew**: `1RM = (100 × Weight) / (52.2 + 41.9 × e^(-0.055 × Reps))`
- **O'Conner**: `1RM = Weight × (1 + Reps/40)`

Accuracy is highest when using 3-10 rep data.

## Success Metrics

- 90-day retention rate above 31%
- Average set logging time under 5 seconds
- Core logging flow in 3 taps or fewer
- App Store rating of 4.5+ stars
- Zero data loss incidents

## License

MIT License - see [LICENSE](LICENSE)
