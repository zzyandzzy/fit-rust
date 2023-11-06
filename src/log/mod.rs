use dotenv::dotenv;
use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    dotenv().expect("Failed to load .env file");
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_time)
        .init();
}
