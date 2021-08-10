use crate::error::{Result, StoreError};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::ToSql, NoTls, Row};

#[derive(Clone)]
pub struct Db {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl Db {
    pub async fn new(addr: impl Into<String>) -> Result<Self> {
        let addr = addr
            .into()
            .parse()
            .map_err(|_| StoreError::InvalidConnAddress)?;
        let manager = PostgresConnectionManager::new(addr, NoTls);
        let pool = Pool::builder()
            .build(manager)
            .await
            .map_err(|_| StoreError::PoolCreationFailure)?;

        Ok(Self { pool })
    }

    pub async fn insert(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<()> {
        let client = self.get_connection().await?;

        client.execute(query, params).await?;

        Ok(())
    }

    pub async fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>> {
        let client = self.get_connection().await?;

        let res = client.query(query, params).await?;

        Ok(res)
    }

    pub async fn query_one(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>> {
        let client = self.get_connection().await?;

        let res = client.query_opt(query, params).await?;

        Ok(res)
    }

    async fn get_connection(
        &self,
    ) -> Result<PooledConnection<'_, PostgresConnectionManager<NoTls>>> {
        self.pool
            .get()
            .await
            .map_err(|_| StoreError::ConnectionFailure)
    }
}
