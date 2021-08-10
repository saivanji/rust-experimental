use postgres::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity<T> {
    id: i32,
    #[serde(flatten)]
    data: T,
}

impl<T> Identity<T> {
    pub fn new(id: i32, data: T) -> Self {
        Self { id, data }
    }

    pub fn entry(&self) -> (i32, &T) {
        (self.id, &self.data)
    }
}

impl<T> From<Row> for Identity<T>
where
    T: From<Row>,
{
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            data: row.into(),
        }
    }
}
