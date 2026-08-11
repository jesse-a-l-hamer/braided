use std::fs::File;
use std::sync::LazyLock;
use tracing::subscriber::{Subscriber, set_global_default};
use tracing_subscriber::{EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt};

fn get_subscriber<Sink>(
    env_filter: String,
    sink: Sink,
    file: Option<File>,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let writer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_level(true)
        .with_writer(sink);
    let file = file.map(|file| {
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_level(true)
            .with_writer(file)
    });

    Registry::default().with(env_filter).with(writer).with(file)
}

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();

    let log_file = if let Ok(log_path) = std::env::var("BRAIDED_LOG_FILE") {
        Some(File::create(log_path).expect("Failed to open log file at {log_path}"))
    } else {
        None
    };
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(default_filter_level, std::io::stdout, log_file);
        set_global_default(subscriber).expect("Failed to set subscriber");
    } else {
        let subscriber = get_subscriber(default_filter_level, std::io::sink, log_file);
        set_global_default(subscriber).expect("Failed to set subscriber");
    };
});

pub fn start_tracing() {
    LazyLock::force(&TRACING);
}
