use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("highlight-card-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HighlightCardMyNoSqlEntity {
    pub highest_total_reward: Option<HighlightCardItemNoSql>,
    pub highest_single_reward: Option<HighlightCardItemNoSql>,
    pub longest_funded_account: Option<HighlightCardItemNoSql>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HighlightCardItemNoSql {
    pub client_name: Option<String>,
    pub country_code: Option<String>,
    pub amount: Option<String>,
}

impl HighlightCardMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "highlight-card"
    }

    pub fn generate_row_key() -> &'static str {
        "settings"
    }
}
