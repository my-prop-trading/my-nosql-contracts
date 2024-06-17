use serde::*;
use service_sdk::rust_extensions::StrOrString;
service_sdk::macros::use_my_no_sql_entity!();
use crate::cfd_cross_margin_bridge::cfd_cross_margin_account_type::CfdCrossMarginAccountTypeMyNoSql;

#[my_no_sql_entity("cfd-cross-margin-bridge-balance-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CfdCrossMarginBridgeBalanceCacheMyNoSqlEntity {
    pub client_id: String,
    pub trader_account_id: String,
    pub platform_account_id: String,
    pub platform_balance: f64,
    pub balance: f64,
    pub equity: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
    pub account_type: CfdCrossMarginAccountTypeMyNoSql,
    pub enabled: bool,
    pub leverage: f64,
}

impl CfdCrossMarginBridgeBalanceCacheMyNoSqlEntity {
    pub fn generate_partition_key<'s>(account_type: i32) -> StrOrString<'s> {
        CfdCrossMarginAccountTypeMyNoSql::from(account_type).to_string().into()
    }

    pub fn generate_row_key<'s>(
        platform_account_id: impl Into<StrOrString<'s>>,
    ) -> StrOrString<'s> {
        platform_account_id.into()
    }
}
