mod worker;

use anyhow::Result;
use simplelog::{Config, LevelFilter, SimpleLogger};
use sources::StackOverflow;
use tokio::time::Duration;
use worker::Worker;

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Config::default())?;

    let worker = Worker::new(vec![StackOverflow::new()]);

    worker.start(Duration::from_secs(3 * 60 * 60)).await?;

    Ok(())
}
