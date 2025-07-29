use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
use service_sdk::rust_extensions::StrOrString;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("files")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FileMyNoSqlEntity {
    pub expires: Timestamp,
    pub bytes: Vec<u8>,
    pub name: String,
    pub owner_id: String,
}

impl FileMyNoSqlEntity {
    pub fn generate_rk<'s>(id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        id.into()
    }

    pub fn generate_pk<'s>(owner_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        owner_id.into()
    }
}
