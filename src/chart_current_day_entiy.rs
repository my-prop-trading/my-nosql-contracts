use rust_extensions::StrOrString;
use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("chart-current-day")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChartCurrentDayMyNoSqlEntity {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl ChartCurrentDayMyNoSqlEntity {
    pub fn generate_partition_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> &'s str {
        trader_account_id
    }

    pub fn generate_row_key() -> StrOrString {
        "CurrentDay"
    }
}