use rust_extensions::StrOrString;
use serde::*;
use crate::common::TraderAccountTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trader-accounts")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderAccountMyNoSqlEntity {
    pub id: String,
    pub title: String,
    pub phase: i32,
    pub platform_id: i32,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub aggregated_id: String,
    pub order_id: String,
    pub client_id: String,
    pub trading_package_id: String,

    pub account_balance: f64,
    pub account_balance_currency: String,
    pub price: f64,
    pub price_currency: String,
    pub leverage: i32,
    pub label: String,

    pub phase1_daily_drawdown: f64,
    pub phase1_overall_drawdown: f64,
    pub phase1_target_profit: f64,
    pub phase1_duration: i32,
    pub phase1_min_trading_days: Option<i32>,
    pub phase1_min_opened_positions: Option<i32>,
    pub phase1_revenue_share: Option<f64>,
    pub phase1_refund: f64,
    pub phase1_attempts: String,

    pub phase2_daily_drawdown: f64,
    pub phase2_overall_drawdown: f64,
    pub phase2_target_profit: f64,
    pub phase2_duration: i32,
    pub phase2_min_trading_days: Option<i32>,
    pub phase2_min_opened_positions: Option<i32>,
    pub phase2_revenue_share: Option<f64>,
    pub phase2_refund: f64,
    pub phase2_attempts: String,

    #[serde(default)]
    pub phase3_daily_drawdown: f64,
    #[serde(default)]
    pub phase3_overall_drawdown: f64,
    #[serde(default)]
    pub phase3_target_profit: f64,
    #[serde(default)]
    pub phase3_duration: i32,
    #[serde(default)]
    pub phase3_min_trading_days: Option<i32>,
    #[serde(default)]
    pub phase3_min_opened_positions: Option<i32>,
    #[serde(default)]
    pub phase3_revenue_share: Option<f64>,
    #[serde(default)]
    pub phase3_refund: f64,
    #[serde(default)]
    pub phase3_attempts: String,

    pub daily_drawdown: f64,
    pub overall_drawdown: f64,

    pub revenue_share: f64,
    pub target_profit: f64,

    pub e_tag: i64,

    pub created_at: i64,
    pub updated_at: i64,

    pub status: TraderAccountStatusMyNoSql,
    pub brand: String,
    pub account_type: TraderAccountTypeMyNoSql,

    #[serde(default)]
    pub trader_package_group_id: String,

    #[serde(default)]
    pub trader_package_group_name: String,

    #[serde(default = "get_default_phase")]
    pub phase_type: TraderPackagePhaseTypMyNoSql,

    #[serde(default)]
    pub refund: f64,

    #[serde(default)]
    //this is an absolute value
    pub refund_granted_by_manager: Option<f64>,

    #[serde(default)]
    pub overall_loss_formula_selector: OverallLossFormulaSelectorMyNoSql,

    #[serde(default)]
    pub daily_loss_formula_selector: DailyLossFormulaSelectorMyNoSql,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum TraderAccountStatusMyNoSql {
    New = 0,
    Active = 1,
    Disabled = 2,
    StageCompleted = 3,
    Blocked = 4,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum TraderPackagePhaseTypMyNoSql {
    Phase1 = 0,
    Phase2 = 1,
    Phase3 = 2,
    InstantFunding = 3,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum DailyLossFormulaSelectorMyNoSql {
    #[default]
    Default = 0,
    DayEntryEquityAndInitialBalance = 1,
    DayEntryEquity = 2,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum OverallLossFormulaSelectorMyNoSql  {
    #[default]
    Default = 0,
    InitialBalance = 1,
    MaxRecordedEquity = 2,
}

use std::convert::TryFrom;

impl TryFrom<i32> for DailyLossFormulaSelectorMyNoSql {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DailyLossFormulaSelectorMyNoSql::Default),
            1 => Ok(DailyLossFormulaSelectorMyNoSql::DayEntryEquityAndInitialBalance),
            2 => Ok(DailyLossFormulaSelectorMyNoSql::DayEntryEquity),
            _ => Err("Invalid value for DailyLossFormulaSelectorMyNoSql"),
        }
    }
}

impl TryFrom<i32> for OverallLossFormulaSelectorMyNoSql {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OverallLossFormulaSelectorMyNoSql::Default),
            1 => Ok(OverallLossFormulaSelectorMyNoSql::InitialBalance),
            2 => Ok(OverallLossFormulaSelectorMyNoSql::MaxRecordedEquity),
            _ => Err("Invalid value for OverallLossFormulaSelectorMyNoSql"),
        }
    }
}

impl TraderAccountMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}

fn get_default_phase() -> TraderPackagePhaseTypMyNoSql {
    TraderPackagePhaseTypMyNoSql::Phase2
}