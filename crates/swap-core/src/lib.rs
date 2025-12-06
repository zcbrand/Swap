//! Swap Core Library
//!
//! Cross-platform business logic for the Swap workout tracking app.
//! This library is compiled for iOS (via UniFFI) and Android,
//! providing consistent workout tracking, 1RM calculations, and session management.

pub mod models;
pub mod one_rm;
pub mod session;
pub mod metrics;
pub mod widget;
pub mod health;
pub mod error;

pub use models::*;
pub use one_rm::*;
pub use session::*;
pub use metrics::*;
pub use widget::*;
pub use health::*;
pub use error::*;

uniffi::setup_scaffolding!();
