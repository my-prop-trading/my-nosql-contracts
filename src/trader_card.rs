use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;
use service_sdk::rust_extensions::StrOrString;
service_sdk::macros::use_my_no_sql_entity!();

/// Cache of the per-trader lifetime Trader Card portion computed by
/// trading-control (core + funded capital/days + highlighted preview).
/// Written by trading-control on cache-miss; read by dashboard-rest-api,
/// which falls back to the gRPC call when the row is absent/expired.
/// PK = client_id, single row per trader. TTL via `expires`.
#[my_no_sql_entity("trader-card-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderCardMyNoSqlEntity {
    pub expires: Timestamp,
    pub core: TraderCardCoreNoSql,
    pub funded: TraderCardFundedNoSql,
    pub highlighted: Vec<TraderCardHighlightedTradeNoSql>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TraderCardCoreNoSql {
    pub total_capital: f64,
    pub total_accounts: u32,
    pub avg_trades_per_day: f64,
    pub most_traded_symbol: String,
    pub lifetime_win_rate: f64,
    pub profit_factor: f64,
    pub max_profit_per_trade: f64,
    pub best_win_streak: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TraderCardFundedNoSql {
    pub unlocked: bool,
    pub funded_capital: f64,
    pub total_days_funded: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TraderCardHighlightedTradeNoSql {
    pub net_profit: f64,
    pub account_size: f64,
    pub account_type: i32,
    pub account_title: String,
    pub duration_sec: i64,
    pub symbol: String,
    pub side: i32,
    pub close_date: i64,
    pub volume: f64,
}

impl TraderCardMyNoSqlEntity {
    pub fn generate_pk<'s>(client_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        client_id.into()
    }

    pub fn generate_rk() -> &'static str {
        "card"
    }
}
