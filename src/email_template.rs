use std::collections::HashMap;
use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("email-template")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EmailTemplateMyNoSqlEntity {
    pub template_name: String,
    pub language: String,
    pub external_id: String,
    pub key_value: HashMap<String, String>,
}

impl EmailTemplateMyNoSqlEntity {
    pub fn generate_partition_key(template_name: &str) -> &str {
        template_name
    }

    pub fn generate_row_key<'s>(language: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        language.into()
    }
}
