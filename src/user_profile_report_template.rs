use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("user-profile-report-templates")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserProfileReportTemplateMyNoSqlEntity {
    pub client_id: Option<String>,
    pub registration_date_from: Option<i64>,
    pub registration_date_to: Option<i64>,
    pub crm_status: Option<i64>,
    pub first_name_reg: Option<String>,
    pub last_name_reg: Option<String>,
    pub first_name_kyc: Option<String>,
    pub last_name_kyc: Option<String>,
    pub email: Option<String>,
    pub manager: Option<String>,
    pub country_reg: Option<String>,
    pub country_reg_ip: Option<String>,
    pub country_poi: Option<String>,
    pub country_poa: Option<String>,
    pub poi_status: Option<i32>,
    pub poa_status: Option<i32>,
    pub kyc_status: Option<i32>,
    pub title: String,
}

impl UserProfileReportTemplateMyNoSqlEntity {
    pub fn generate_partition_key<'s>(officer_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        officer_id.into()
    }

    pub fn generate_row_key<'s>(template_title: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        template_title.into()
    }
}
