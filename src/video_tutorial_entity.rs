use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("video-tutorials")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct VideoTutorialEntity {
    pub link: String,
    pub title: String,
    pub category: String,
    pub duration: String,
    pub description: String,
}

impl VideoTutorialEntity {
    pub fn generate_partition_key() -> &'static str {
        "video-tutorials"
    }

    pub fn generate_row_key<'s>(id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        id.into()
    }
}
