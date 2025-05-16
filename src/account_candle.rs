use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
use service_sdk::rust_extensions::StrOrString;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("account-candle-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountCandleMyNoSqlEntity {
    pub interval_date: String,
    pub interval: i32,
    pub balance_data: AccountCandleDataMyNoSqlEntity,
    pub equity_data: AccountCandleDataMyNoSqlEntity,
    pub pnl_data: AccountCandleDataMyNoSqlEntity,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountCandleDataMyNoSqlEntity {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub low_after_high: f64,
    pub timestamp: i64,
}

impl AccountCandleMyNoSqlEntity {
    pub fn generate_rk<'s>(candle_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        candle_id.into()
    }

    pub fn generate_pk<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }

    pub fn get_trader_account_id(&self) -> &str {
        &self.row_key
    }

    pub fn get_candle_interval(&self) -> i32 {
        self.row_key.parse().unwrap()
    }
}
