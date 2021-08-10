use derive_more::Display;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Display, Debug, ToSql, FromSql, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[postgres(name = "source")]
pub enum SourceType {
    #[postgres(name = "stackoverflow")]
    #[display(fmt = "stackoverflow")]
    StackOverflow,
}
