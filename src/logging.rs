use tracing_subscriber::{
    EnvFilter,
    fmt::{format::FmtSpan, time::Uptime},
    prelude::*,
};

/// Sets up enhanced logging with better syntax highlighting and formatting
pub fn setup_logging() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a custom formatter for better visualization
    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_timer(Uptime::default())
        .with_ansi(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true);

    // Create the environment filter for controlling log levels
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info,axum_server=debug,rmcp=debug,axum_server::services::mcp_client=debug")
    });

    // Register the subscriber with our custom layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .try_init()?;

    Ok(())
}

/// Creates a middleware layer that adds tracing to the HTTP request/response cycle
pub fn trace_layer() -> tower_http::trace::TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
> {
    tower_http::trace::TraceLayer::new_for_http()
}
