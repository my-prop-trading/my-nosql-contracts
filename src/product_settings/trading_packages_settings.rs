use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"settings", row_key: "trading-packages")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPackagesSettingsModel {
    pub use_groups: bool,
}