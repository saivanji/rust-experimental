use anyhow::Result;
use common::Source;
use log::{error, info};
use std::env;
use std::sync::Arc;
use store::Store;
use tokio::time::{self, Duration};

pub struct Worker<T: Source> {
    sources: Vec<T>,
}

impl<T: Source + 'static> Worker<T> {
    pub fn new(sources: Vec<T>) -> Self {
        Self { sources }
    }

    pub async fn start(&self, duration: Duration) -> Result<()> {
        let mut interval = time::interval(duration);

        let addr = env::var("DATABASE_URL")?;
        let store = Arc::new(Store::new(addr).await?);

        for source in &self.sources {
            loop {
                interval.tick().await;

                info!("parsing {}", source.kind());

                for page in source.clone() {
                    for job in page {
                        if let Err(err) = store.insert_job(&job).await {
                            error!("failed to insert job - {}", err);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
