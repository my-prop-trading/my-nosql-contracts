use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"settings", row_key: "google-analytics")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GoogleAnalyticsSettingsModel {
    pub monitoring_events: Vec<String>,
}