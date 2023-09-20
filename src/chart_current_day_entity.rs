use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("chart-current-day")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChartCurrentDayMyNoSqlEntity {
    pub balance: CandleMyNoSqlEntity,
    pub equity: CandleMyNoSqlEntity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CandleMyNoSqlEntity {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl ChartCurrentDayMyNoSqlEntity {
    pub fn generate_partition_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }

    pub fn generate_row_key() -> StrOrString<'static> {
        "CurrentDay".into()
    }
}