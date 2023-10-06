use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("company-properties")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyPropertiesMyNoSqlEntity {
    pub company_name: String,
}

impl CompanyPropertiesMyNoSqlEntity {
    pub fn generate_partition_key<'s>(brand_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        brand_id.into()
    }

    pub fn generate_row_key<'s>() -> StrOrString<'s> {
        "Company".into()
    }
}
