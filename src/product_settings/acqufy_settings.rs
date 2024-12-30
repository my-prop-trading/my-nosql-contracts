use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"affiliate", row_key: "acqufy")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AcqufySettingsModel {
    pub api_key: String,
    pub affiliate_url: String,
    pub traffic_management_url: String,
    pub server_tracking_url: String,
}