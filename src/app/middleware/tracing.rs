use tracing_subscriber::{
    fmt,
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt
};

pub fn init_tracing_layer() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(true)
                .with_line_number(true)
        )
        .init()
}