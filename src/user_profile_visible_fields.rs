use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("user-profile-visible-fields")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetUserProfileVisibleFieldsMyNoSqlEntity {
    pub registration_date_from: bool,
    pub registration_date_to: bool,
    pub crm_status: bool,
    pub first_name_reg: bool,
    pub last_name_reg: bool,
    pub first_name_kyc: bool,
    pub last_name_kyc: bool,
    pub email: bool,
    pub manager: bool,
    pub country_reg: bool,
    pub country_reg_ip: bool,
    pub country_poi: bool,
    pub country_poa: bool,
    pub poi_status: bool,
    pub poa_status: bool,
    pub kyc_status: bool,
    pub client_id: bool,
}

impl GetUserProfileVisibleFieldsMyNoSqlEntity {
    pub fn generate_partition_key<'s>(officer_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        officer_id.into()
    }

    pub fn generate_row_key<'s>(template_title: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        template_title.into()
    }

    pub fn generate_default_row_key<'s>() -> StrOrString<'s> {
        "UserProfileVisibleFields".into()
    }
}
