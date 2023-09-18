use rust_extensions::StrOrString;
use serde::{Deserialize, Serialize};
use crate::broker_no_sql::BrokerMyNoSql;
use crate::trading_platform_no_sql::TradingPlatformMyNoSql;

#[my_no_sql_macros::my_no_sql_entity("trader-accounts")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderAccountMyNoSqlEntity {
    pub id: String,
    pub title: String,
    pub phase: i32,
    pub broker: BrokerMyNoSql,
    pub trading_platform: TradingPlatformMyNoSql,
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
pub enum TraderAccountTypeMyNoSql {
    Demo = 0,
    Live = 1,
}

impl TraderAccountMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}
