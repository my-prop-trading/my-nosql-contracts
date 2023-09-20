use serde::*;
use service_sdk::rust_extensions::StrOrString;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("dashboard-brief-statistics")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DashboardBriefStatisticsMyNoSqlEntity {
    pub average_win: f64,
    pub average_loss: f64,
    pub win_ratio: f64,
    pub risk_reward: f64,
    pub days_traded: u32,
    pub min_trading_days: u32,
    pub traded_days_percent: f64,
    pub current_profit: f64,
    pub target_profit: f64,
    pub profit_percent: f64,
    pub max_daily_loss: f64,
    pub max_daily_loss_recorded: f64,
    pub max_daily_loss_percent: f64,
    pub max_overall_loss: f64,
    pub max_overall_loss_recorded: f64,
    pub max_overall_loss_percent: f64,
    pub overall_profit_percent: f64,
}

impl DashboardBriefStatisticsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}