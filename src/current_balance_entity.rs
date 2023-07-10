use rust_extensions::StrOrString;
use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("current-balance")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentBalanceMyNoSqlEntity {
    pub balance: f64,
    pub equity: f64,
}

impl CurrentBalanceMyNoSqlEntity {
    pub fn generate_partition_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }

    pub fn generate_row_key() -> StrOrString<'static> {
        "CurrentBalance".into()
    }
}