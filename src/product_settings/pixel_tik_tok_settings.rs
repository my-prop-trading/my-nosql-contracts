use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"affiliate", row_key: "pixeltiktok")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PixelTikTokSettingsModel {
    pub id: Option<String>,
    pub token: Option<String>,
    pub hide: Option<bool>,
}