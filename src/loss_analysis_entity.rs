use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("loss-analysis")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LossAnalysisMyNoSqlEntity {
    pub initial_account_balance: f64,
    pub overall_loss_level: f64,
    pub initial_account_balance_loss: f64,
    pub current_equity: f64,
    pub current_balance: f64,
    pub initial_account_balance_loss_percent: f64,
    pub daily_loss_level: f64,
    pub daily_loss_percent: f64,
    #[serde(default)]
    pub current_daily_loss: f64,
}

impl LossAnalysisMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}