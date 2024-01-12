use rust_extensions::StrOrString;
use crate::broker_no_sql::BrokerMyNoSql;
use crate::trading_platform_no_sql::TradingPlatformMyNoSql;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("woo-commerce-product-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WoCommerceProductMappingMyNoSqlEntity {
    pub product_id: i32,
}

impl WoCommerceProductMappingMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}
