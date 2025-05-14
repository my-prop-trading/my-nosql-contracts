use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
use service_sdk::rust_extensions::StrOrString;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("account-candle-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountCandleMyNoSqlEntity {
    pub expires: Timestamp,
    pub interval_date: String,
    pub balance_data: CandleDataMyNoSqlEntity,
    pub equity_data: CandleDataMyNoSqlEntity,
    pub pnl_data: CandleDataMyNoSqlEntity,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CandleDataMyNoSqlEntity {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub low_after_high: f64,
    pub timestamp: i64,
}

impl AccountCandleMyNoSqlEntity {
    pub fn generate_pk(candle_interval: i32) -> String {
        candle_interval.to_string()
    }

    pub fn generate_rk<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }

    pub fn get_trader_account_id(&self) -> &str {
        &self.row_key
    }

    pub fn get_candle_interval(&self) -> i32 {
        self.partition_key.parse().unwrap()
    }
}
