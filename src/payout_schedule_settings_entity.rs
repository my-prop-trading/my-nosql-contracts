use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("payout-schedule-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayoutScheduleSettingsMyNoSqlEntity {
    pub first_payout_in_days: i64,

    pub further_payouts_in_days: i64,
}

impl PayoutScheduleSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key() -> &'static str {
        "payout-schedule-settings"
    }
}
