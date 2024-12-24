use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"affiliate", row_key: "meta-pixel")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MetaPixelSettingsModel {
    pub access_token: String,
    pub pixel_id: String,
}