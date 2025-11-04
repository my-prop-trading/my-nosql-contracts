use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;
use service_sdk::rust_extensions::StrOrString;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trading-metrics-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingMetricsMyNoSqlEntity {
    pub expires: Timestamp,
    pub average_win: f64,
    pub average_loss: f64,
    pub win_ratio: f64,
    pub risk_ratio: f64,
    pub traded_days_count: usize,
    pub traded_days_percent: f64,
    pub current_total_net_profit: f64,
    pub profit_percent: f64,
    pub max_daily_loss: f64,
    pub max_daily_loss_recorded: f64,
    pub max_daily_loss_percent: f64,
    pub max_overall_loss_recorded: f64,
    pub max_overall_loss_percent: f64,
    pub min_trading_days_count: usize,
    pub target_profit: f64,
    pub max_overall_loss: f64,
    pub overall_profit_percent: f64,
    pub initial_balance: f64,
    pub overall_loss_level: f64,
    pub initial_balance_loss: f64,
    pub current_equity: f64,
    pub current_balance: f64,
    pub initial_balance_loss_percent: f64,
    pub daily_loss_level: f64,
    pub daily_loss_percent: f64,
    pub current_daily_loss: f64,
    pub day_entry_equity: f64,
    pub best_day_profit: f64,
    pub best_day_ratio: f64,
    pub best_day_cap: f64,
    pub best_day_rule_status: i32,
}

impl TradingMetricsMyNoSqlEntity {
    pub fn generate_pk<'s>(client_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        client_id.into()
    }

    pub fn generate_rk<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}
