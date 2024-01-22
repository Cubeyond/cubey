use tokio::time::Instant;
use tracing::{error, info, level_filters::LevelFilter, subscriber, warn};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

pub fn initialise_subscriber() {
  let start_time = Instant::now();

  let rust_log = match dotenvy::var("RUST_LOG") {
      Ok(level) => level,
      Err(_) => {
          error!("Could not get log level from .env file, setting default...");
          format!("info")
      }
  };

  let filter = match EnvFilter::try_new(format!("cubey={rust_log}")) {
      Ok(filter) => filter,
      Err(_) => {
          error!("Could not get filter from .env file, setting default...");
          EnvFilter::default()
      }
  };

  let subscriber = Subscriber::builder()
      .with_max_level(LevelFilter::TRACE)
      .with_env_filter(filter)
      .compact()
      .finish();

  if let Err(_) = subscriber::set_global_default(subscriber) {
      warn!("Could not set custom subscriber, setting default...");

      let default_subscriber = Subscriber::default();
      let _ = subscriber::set_global_default(default_subscriber);
  }

  let elapsed_time = start_time.elapsed();
  info!("Initalised logger in {elapsed_time:.2?}");
}