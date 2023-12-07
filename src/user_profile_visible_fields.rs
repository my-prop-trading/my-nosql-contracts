use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("user-profile-visible-fields")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetUserProfileVisibleFieldsMyNoSqlEntity {
    registration_date_from: bool,
    registration_date_to: bool,
    crm_status: bool,
    first_name_reg: bool,
    last_name_reg: bool,
    first_name_kyc: bool,
    last_name_kyc: bool,
    email: bool,
    manager: bool,
    country_reg: bool,
    country_reg_ip: bool,
    country_poi: bool,
    country_poa: bool,
    poi_status: bool,
    poa_status: bool,
    kyc_status: bool,
    client_id: bool,
}

impl GetUserProfileVisibleFieldsMyNoSqlEntity {
    pub fn generate_partition_key<'s>(officer_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        officer_id.into()
    }

    pub fn generate_row_key<'s>() -> StrOrString<'s> {
        "UserProfileVisibleFields".into()
    }
}
