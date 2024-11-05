use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-platform")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPlatformSettingsModel {
    pub platform_id: i32,
    pub platform_name: String,
    pub platform_short_name: String,
    pub platform_icon_url: Option<String>,
    pub platform_links: Option<PlatformSettingsLinks>,
    pub enabled: bool,
    pub platform_url: String,
    pub platform_credential_type: Option<i32>,
    pub hide_platform_url_for_dashboard: Option<bool>,
    pub platform_group_demo: PlatformSettingsGroup,
    pub platform_group_live: PlatformSettingsGroup,
}

impl TradingPlatformSettingsModel {
    pub fn get_trading_platform_id(
        &self,
    ) -> i32 {
        self.platform_id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformSettingsLinks {
    pub windows: Option<String>,
    pub mac: Option<String>,
    pub ios: Option<String>,
    pub android: Option<String>,
    pub web: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformSettingsGroup {
    pub group_name_default: String,
    pub groups: Vec<String>
}

