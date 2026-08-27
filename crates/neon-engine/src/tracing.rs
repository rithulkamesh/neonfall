use tracing_error::ErrorLayer;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};


pub fn install() {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            [
                "info",
                "neon_engine=info",
                "neon_renderer=info",
                "wgpu_core=warn",
                "wgpu_hal=warn",
                "naga=warn",
            ]
            .join(",")
            .parse()
            .unwrap()
        });

        let json = std::env::var("NEON_LOG_FORMAT")
            .map(|v| v.eq_ignore_ascii_case("json"))
            .unwrap_or(false);

        if json {
            install_json(filter);
        } else {
            install_pretty(filter);
        }
}

fn try_init(result: Result<(), tracing_subscriber::util::TryInitError>) {
    if let Err(e) = result {
        eprintln!("tracing already initialized: {e}");
    }
}

fn install_json(filter: EnvFilter) {
    try_init(
        tracing_subscriber::registry()
            .with(filter)
            .with(ErrorLayer::default())
            .with(
                fmt::layer()
                    .json()
                    .with_span_list(true)
                    .with_current_span(true)
                    .with_span_events(FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(true),
            )
            .try_init(),
    );
}

fn install_pretty(filter: EnvFilter) {
    try_init(
        tracing_subscriber::registry()
            .with(filter)
            .with(ErrorLayer::default())
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_span_events(FmtSpan::CLOSE)
                    .with_level(true)
                    .compact(),
            )
            .try_init(),
    );
}
