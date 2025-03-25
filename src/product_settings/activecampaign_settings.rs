use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"affiliate", row_key: "activecampaign")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveCampaignSettingsModel {
    pub api_key: String,
    pub url: String,
    pub event_actid: String,
    pub event_key: String,
    pub backoffice_url: String,
    pub default_account_owner: String,
}