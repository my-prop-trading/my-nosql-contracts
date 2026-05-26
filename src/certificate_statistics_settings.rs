
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("certificate-statistics-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CertificateStatisticsSettingsMyNoSqlEntity {
    pub period_days: u32,
    pub quantity: u32,
}

impl CertificateStatisticsSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "*"
    }

    pub fn generate_row_key() -> &'static str {
        "*"
    }
}
