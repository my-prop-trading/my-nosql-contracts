use rust_extensions::StrOrString;
use serde::*;
use crate::common::TraderAccountTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("intraday-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntradaySettingsNoSqlEntity {
    pub platform_id: i32,
    pub hours_start: i32,
    pub minutes_start: i32,
    pub hours_end: i32,
    pub minutes_end: i32,
}

impl IntradaySettingsNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(platform_id: i32) -> StrOrString<'s> {
        platform_id.to_string().into()
    }
}

#[my_no_sql_entity("intraday-restrictions")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntradayRestrictionsNoSqlEntity {
    pub account_type: TraderAccountTypeMyNoSql,
}

impl IntradayRestrictionsNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>() -> StrOrString<'s> {
        "intraday-restrictions".into()
    }
}

#[my_no_sql_entity("intraday-filter")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntradayFilterNoSqlEntity {
    pub enabled: bool,
}

impl IntradayFilterNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>() -> StrOrString<'s> {
        "intraday-filter".into()
    }
}