use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"kyc-provider", row_key: "identomat")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IdentomatSettingsModel {
    pub api_key: String,
    pub iframe_url: String,
}