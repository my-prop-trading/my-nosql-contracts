use std::collections::HashMap;
use service_sdk::rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("language-content")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LanguageContentMyNoSqlEntity {
    pub content: HashMap<String, String>,
    pub local_language_name: String,
    pub language_name: String, 
    pub language_iso2: String,
    pub enabled: bool
}

impl LanguageContentMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "language"
    }

    pub fn generate_row_key<'s>(language_iso2: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        language_iso2.into()
    }
}
