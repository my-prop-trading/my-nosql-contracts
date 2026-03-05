use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("banners")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BannerEntity {
    pub title: String,
    pub is_enable: bool,
    pub description: String,
}

impl BannerEntity {
    pub fn generate_partition_key() -> &'static str {
        "*"
    }

    pub fn generate_row_key() -> &'static str {
        "banners"
    }
}
