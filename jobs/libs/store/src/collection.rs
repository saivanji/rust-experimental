use crate::error::Result;
use crate::Db;
use common::Identity;
use serde::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Row};

#[derive(Serialize, Deserialize)]
pub struct Collection<T> {
    items: Vec<Identity<T>>,
    total: i64,
}

impl<T> Collection<T>
where
    T: From<Row>,
{
    pub async fn fetch(
        total_query: &str,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
        db: &Db,
    ) -> Result<Self> {
        let items = db
            .query(query, params)
            .await?
            .into_iter()
            .map(Identity::from)
            .collect();
        let total = db
            .query_one(total_query, &[])
            .await
            .map(|row| row.unwrap().get("total"))?;

        Ok(Self { items, total })
    }
}
