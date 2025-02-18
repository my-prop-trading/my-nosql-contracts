use serde::{Deserialize, Serialize};
use service_sdk::rust_extensions::StrOrString;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trader-account-http-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderAccountCacheMyNoSqlEntity {
    pub trader_account_id: String,
    pub aggregate_id: String,
    pub title: String,
    pub phase: i32,
    pub platform_id: i32,
    pub platform_name: String,
    pub platform_type: i32,
    pub platform_short_name: String,
    pub platform_icon_url: Option<String>,
    pub platform_credential_type: i32,
    pub balance: f64,
    pub start_date: String,
    pub end_date: String,
    pub created_at: String,
    pub updated_at: String,
    pub status: i32,
    pub status_text: i32,
    pub account_type: i32,
    pub order_id: String,
    pub blocker_type: Option<i32>,
    pub trader_package_group_id: String,
    pub trader_package_group_name: String,
    pub initial_account_currency: String,
    pub server: String,
    pub trader_package_phase_type_model: i32,
    pub news_trading_restriction_trades: Option<Vec<String>>,
    pub hide_platform_url_for_dashboard: bool,
    pub trader_package_type: i32,
    pub trading_condition: i32,
}


impl TraderAccountCacheMyNoSqlEntity {
    pub fn generate_pk<'s>(client_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        client_id.into()
    }

    pub fn generate_rk<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}
