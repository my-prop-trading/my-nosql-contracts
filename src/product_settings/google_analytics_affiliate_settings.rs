use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"affiliate", row_key: "google-analytics")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GoogleAnalyticsAffiliateSettingsModel {
    pub api_key: String,
    pub affiliate_url: String,
    pub measurement_id: String,
}