pub use tracing;
/// Reexport
pub use tracing::*;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug)]
pub enum LoggingFormat {
    Json,
    Plain,
}

pub fn init_tracing(logging_format: Option<LoggingFormat>) {
    // let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    // let formatting_layer = BunyanFormattingLayer::new(app_name.to_owned(), non_blocking_writer);
    let env_layer = EnvFilter::from_default_env();

    let timer = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let logging_format = logging_format.unwrap_or(LoggingFormat::Plain);
    if let LoggingFormat::Json = logging_format {
        tracing_subscriber::registry()
            // .with(opentelemetry)
            .with(fmt::Layer::default().with_timer(timer).json())
            .with(env_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            // .with(opentelemetry)
            .with(fmt::Layer::default().with_timer(timer))
            .with(env_layer)
            .init();
    }
}
