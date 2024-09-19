use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-platform-bridge", row_key: "trade-locker")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct TradeLockerSettingsModel {
    pub api_url: String,
    pub api_key: String,
}
