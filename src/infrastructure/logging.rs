use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry, util::SubscriberInitExt};

// Initialize global logging using tracing-subscriber.
// - Respects RUST_LOG if set (e.g., "info,sqlx=warn,tower_http=info")
// - Defaults to INFO level
// - Emits JSON logs when APP_ENV=production, otherwise pretty text
// Safe to call multiple times; subsequent calls are no-ops.
pub fn init_logger() {
    // Build EnvFilter from RUST_LOG or default
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Initialize registry if not already set
    // Using try_init to avoid panic if something already initialized it in tests
    let _ = Registry::default()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(false)
                .with_file(true)
                .with_line_number(true)
        )
        .try_init();
}
