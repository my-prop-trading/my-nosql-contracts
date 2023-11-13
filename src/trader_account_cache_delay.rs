use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trader-account-cache-delay")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderAccountCacheDelayMyNoSqlEntity {
    pub trader_account_id: String,
    pub time_interval_before_cache_update: i64,
    pub last_time_updated: Option<i64>,
}

impl TraderAccountCacheDelayMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}