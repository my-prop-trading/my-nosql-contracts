use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::enum_model;

#[enum_model(partition_key:"trading-platform-bridge", row_key: "cross-margin")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CrossMarginSettingsModel {
    pub group_demo: String,
    pub group_live: String,
    pub integration_bridge_grpc_service_url: String,
    pub integration_api_key_encrypted: String,
}
