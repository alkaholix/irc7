//! IRC7 Logging infrastructure
//!
//! This crate provides logging functionality for IRC7.

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// Initialize the logging system
pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .finish();

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Warning: Failed to set global default subscriber");
    }
}

/// Initialize logging with a custom level
pub fn init_with_level(level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .finish();

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Warning: Failed to set global default subscriber");
    }
}

/// Attach logging to the current context
pub fn attach() {
    init();
}
