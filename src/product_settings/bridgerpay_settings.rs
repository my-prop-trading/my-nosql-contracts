use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"payment-bridge", row_key: "bridgerpay")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct BridgerpaySettingsModel {
    pub api_url: String,
    pub api_key: String,
    pub user_name: String,
    pub password: String,
    pub cashier_key: String,
}