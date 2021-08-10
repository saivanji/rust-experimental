use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[postgres(name = "remote")]
pub enum Remote {
    #[postgres(name = "yes")]
    Yes,
    #[postgres(name = "no")]
    No,
    #[postgres(name = "partial")]
    Partial,
    #[postgres(name = "unknown")]
    Unknown,
}
