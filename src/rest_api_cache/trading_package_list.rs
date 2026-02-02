use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trading-package-list-api-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPackageListApiCacheMyNoSqlEntity {
    pub expires: Timestamp,
    pub brokers: Vec<BrokerApiCacheMyNoSqlEntity>,
    pub orders: Vec<TradingPackageFieldsOrderApiCacheMyNoSqlEntity>,
    pub packages: Vec<TradingPackageApiCacheMyNoSqlEntity>,
    pub groups: Vec<TradingPackageGroupApiCacheMyNoSqlEntity>,
    pub platforms: Vec<PlatformDetailApiCacheMyNoSqlEntity>,
    pub use_groups: bool,
    pub use_trader_package_type: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BrokerApiCacheMyNoSqlEntity {
    pub name: String,
    pub id: i32,
    pub ico_url: String,
    pub platforms: Vec<PlatformApiCacheMyNoSqlEntity>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformApiCacheMyNoSqlEntity {
    pub name: String,
    pub id: i32,
    pub platform_type: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPackageFieldsOrderApiCacheMyNoSqlEntity {
    pub id: String,
    pub phase1_daily_drawdown: Option<i32>,
    pub phase1_overall_drawdown: Option<i32>,
    pub phase1_target_profit: Option<i32>,
    pub phase1_duration: Option<i32>,
    pub phase1_min_trading_days: Option<i32>,
    pub phase1_min_opened_positions: Option<i32>,
    pub phase1_attempts: Option<i32>,
    pub phase2_daily_drawdown: Option<i32>,
    pub phase2_overall_drawdown: Option<i32>,
    pub phase2_target_profit: Option<i32>,
    pub phase2_duration: Option<i32>,
    pub phase2_min_trading_days: Option<i32>,
    pub phase2_min_opened_positions: Option<i32>,
    pub phase2_attempts: Option<i32>,
    pub phase3_daily_drawdown: Option<i32>,
    pub phase3_overall_drawdown: Option<i32>,
    pub phase3_target_profit: Option<i32>,
    pub phase3_duration: Option<i32>,
    pub phase3_min_trading_days: Option<i32>,
    pub phase3_min_opened_positions: Option<i32>,
    pub phase3_revenue_share: Option<i32>,
    pub daily_drawdown: Option<i32>,
    pub overall_drawdown: Option<i32>,
    pub revenue_share: Option<i32>,
    pub target_profit: Option<i32>,
    pub refund: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPackageApiCacheMyNoSqlEntity {
    pub id: String,
    pub title: String,
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
    pub phase1_best_day_cap: f64,
    pub phase1_min_opened_positions: Option<i32>,
    pub phase1_attempts: Vec<TradingPackageAttemptApiCacheMyNoSqlEntity>,
    pub phase2_daily_drawdown: f64,
    pub phase2_overall_drawdown: f64,
    pub phase2_target_profit: f64,
    pub phase2_duration: i32,
    pub phase2_min_trading_days: Option<i32>,
    pub phase2_best_day_cap: f64,
    pub phase2_min_opened_positions: Option<i32>,
    pub phase2_attempts: Vec<TradingPackageAttemptApiCacheMyNoSqlEntity>,
    pub phase3_daily_drawdown: f64,
    pub phase3_overall_drawdown: f64,
    pub phase3_target_profit: f64,
    pub phase3_duration: i32,
    pub phase3_min_trading_days: Option<i32>,
    pub phase3_best_day_cap: f64,
    pub phase3_min_opened_positions: Option<i32>,
    pub phase3_attempts: Vec<TradingPackageAttemptApiCacheMyNoSqlEntity>,
    pub daily_drawdown: f64,
    pub overall_drawdown: f64,
    pub revenue_share: f64,
    pub target_profit: f64,
    pub refund: f64,
    pub best_day_cap: f64,
    pub trader_package_phase_type_model: i32,
    pub trader_package_type: i32,
    pub trading_condition: i32,
    pub refund_payout_number: i32,
    pub compensation_type: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPackageAttemptApiCacheMyNoSqlEntity {
    pub id: String,

    #[serde(rename = "tradingPackageId")]
    pub trading_package_id: String,
    pub price: f64,

    #[serde(rename = "priceCurrency")]
    pub price_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPackageGroupApiCacheMyNoSqlEntity {
    pub name: String,
    pub id: String,
    pub ico_url: String,
    pub order: i32,
    pub packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformDetailApiCacheMyNoSqlEntity {
    pub name: String,
    pub id: i32,
    pub platform_short_name: String,
    pub platform_icon_url: Option<String>,
    pub restricted_countries: Vec<String>,
}

impl crate::TradingPackageListApiCacheMyNoSqlEntity {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(country_code: impl Into<String>) -> String {
        country_code.into()
    }
}
