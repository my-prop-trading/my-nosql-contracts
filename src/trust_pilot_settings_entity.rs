use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trust-pilot-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TrustPilotSettingsMyNoSqlEntity {
    pub url: String,
    pub key: String,
}

impl TrustPilotSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key() -> &'static str {
        "trust-pilot-settings"
    }
}
