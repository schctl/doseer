//! Application wide logging configurations.

use tracing_subscriber::filter::EnvFilter;

/// Initialize global tracing subscriber. Call only once.
pub fn init_tracing() {
    let mut env_filter = EnvFilter::from_default_env()
        .add_directive("doseer=info".parse().unwrap())
        .add_directive("iced=error".parse().unwrap());

    if cfg!(feature = "debug") {
        env_filter = env_filter
            .add_directive("doseer=debug".parse().unwrap())
            .add_directive("iced=warn".parse().unwrap());
    } else if cfg!(feature = "trace") {
        env_filter = env_filter
            .add_directive("doseer=trace".parse().unwrap())
            .add_directive("iced=info".parse().unwrap());
    }

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
