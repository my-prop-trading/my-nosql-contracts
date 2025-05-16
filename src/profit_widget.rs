use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("profit-widget-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ProfitWidgetMyNoSqlEntity {
    pub expires: Timestamp,
    pub withdrawn_profit: f64,
    pub pending_profit: f64,
    pub refund_fee_amount: f64,
    pub current_profit: f64,
    pub left_to_withdraw_in_profit_period: f64,
    pub trader_account_revenue_share_percentage: f64,
}

impl ProfitWidgetMyNoSqlEntity {
    pub fn generate_pk(client_id: impl Into<String>) -> String {
        client_id.into()
    }

    pub fn generate_rk(trader_account_id: impl Into<String>) -> String {
        trader_account_id.into()
    }
}
