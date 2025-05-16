use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("platform-info-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformInfoMyNoSqlEntity {
    pub platform_id: i32,
    pub platform_name: String,
    pub platform_icon_url: Option<String>,
    pub platform_links: Option<PlatformLinksMyNoSqlEntity>,
    pub broker_name: Option<String>,
    pub broker_icon_url: Option<String>,
    pub enabled: bool,
    pub platform_url: String,
    pub broker_id: i32,
    pub platform_type: i32,
    pub platform_short_name: String,
    pub credential_type: i32,
    pub hide_platform_url_for_dashboard: bool,
    pub restricted_countries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformLinksMyNoSqlEntity {
    pub windows: Option<String>,
    pub mac: Option<String>,
    pub ios: Option<String>,
    pub android: Option<String>,
    pub web: Option<String>,
}

impl PlatformInfoMyNoSqlEntity {
    pub fn generate_pk() -> String {
        "*".to_string()
    }

    pub fn generate_rk(platform_id: i32) -> String {
        platform_id.to_string()
    }
}
