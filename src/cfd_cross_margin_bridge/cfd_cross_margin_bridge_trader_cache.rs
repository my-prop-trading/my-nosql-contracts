use serde::*;
use service_sdk::rust_extensions::StrOrString;
service_sdk::macros::use_my_no_sql_entity!();
use crate::cfd_cross_margin_bridge::cfd_cross_margin_account_type::CfdCrossMarginAccountTypeMyNoSql;

#[my_no_sql_entity("cfd-cross-margin-bridge-trader-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CfdCrossMarginBridgeTraderCacheMyNoSqlEntity {
    pub client_id: String,
    pub trader_account_id: String,
    pub platform_account_id: String,
    pub account_type: CfdCrossMarginAccountTypeMyNoSql,
}

impl CfdCrossMarginBridgeTraderCacheMyNoSqlEntity {
    pub fn generate_partition_key<'s>(client_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        client_id.into()
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}
