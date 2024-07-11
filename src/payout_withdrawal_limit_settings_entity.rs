use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("payout-withdrawal-limit-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayoutWithdrawalLimitSettingsMyNoSqlEntity {
    pub value: f64,

    #[serde(default)]
    pub is_enabled: bool,
}

impl PayoutWithdrawalLimitSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key() -> &'static str {
        "payout-withdrawal-limit-settings"
    }
}