use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("personal-data-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PersonalDataMyNoSqlEntity {
    pub expires: Timestamp,

    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
    pub date_of_birth: Option<i64>,
    pub country_of_residence: Option<String>,
    pub zip_code: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub sex: Option<i32>,
    pub account_type: Option<i32>,
    pub created_date: Option<i64>,
    pub region: Option<String>,
    pub country_of_registration: Option<String>,
    pub country_of_registration_by_ip: Option<String>,
}

impl PersonalDataMyNoSqlEntity {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: impl Into<String>) -> String {
        id.into()
    }
    
    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
