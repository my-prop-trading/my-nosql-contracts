use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-platform-bridge", row_key: "ctrader")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CtraderSettingsModel {
    pub demo_manager_login: i64,
    pub demo_manager_password_encrypted: String,
    pub live_manager_login: i64,
    pub live_manager_password_encrypted: String,
    pub broker_name: String,
    pub plant_id: String,
    pub demo_webservices_api_url: String,
    pub live_webservices_api_url: String,
    pub enabled_account_group_name: String,
    pub disabled_account_group_name: String,
    pub demo_environment_name: String,
    pub live_environment_name: String,
    pub demo_manager_api_url: String,
    pub live_manager_api_url: String,
}
