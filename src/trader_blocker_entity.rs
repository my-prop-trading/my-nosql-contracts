use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trader-blockers")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderBlockerMyNoSqlEntity {
    pub client_id: String,
    pub created_by: String,
    pub created_at: i64,
    pub block_type: BlockerTypeMyNoSql,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum BlockerTypeMyNoSql {
    BlockTrades = 1,
    BlockTradesAndFuture = 2,
    BlockTradesAndFutureAndCurrent = 4,
    BlockTradesForPayouts = 5,
}

impl TraderBlockerMyNoSqlEntity {
    pub fn generate_partition_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }

    pub fn generate_row_key<'s>() -> StrOrString<'s> {
        "Blocker".into()
    }
}
