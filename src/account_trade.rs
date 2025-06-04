use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;
use service_sdk::rust_extensions::StrOrString;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("account-trade-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountTradeMyNoSqlEntity {
    pub expires: Timestamp,

    pub client_id: String,
    pub platform_account_id: String,
    pub symbol: String,
    pub digits: i64,
    pub trade_type: i32,
    pub lots: f64,
    pub open_time: i64,
    pub open_price: f64,
    pub sl: f64,
    pub tp: f64,
    pub close_time: Option<i64>,
    pub fee: f64,
    pub swap: f64,
    pub close_price: f64,
    pub profit: f64,
    pub taxes: f64,
    pub comment: String,
    pub timestamp: i64,
    pub platform_id: i32,
    pub account_type: i32,
}

impl AccountTradeMyNoSqlEntity {
    pub fn generate_rk<'s>(item_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        item_id.into()
    }

    pub fn generate_pk<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }

    pub fn get_trader_account_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_trade_id(&self) -> &str {
        &self.row_key
    }
}
