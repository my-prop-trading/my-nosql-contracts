use service_sdk::rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trader-packages-field-order")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderPackageFieldsOrderMyNoSqlEntity {
    pub phase1_daily_drawdown: Option<i32>,
    pub phase1_overall_drawdown: Option<i32>,
    pub phase1_target_profit: Option<i32>,
    pub phase1_duration: Option<i32>,
    pub phase1_min_trading_days: Option<i32>,
    pub phase1_min_opened_positions: Option<i32>,
    pub phase1_revenue_share: Option<i32>,
    pub phase1_refund: Option<i32>,
    pub phase1_attempts: Option<i32>,

    pub phase2_daily_drawdown: Option<i32>,
    pub phase2_overall_drawdown: Option<i32>,
    pub phase2_target_profit: Option<i32>,
    pub phase2_duration: Option<i32>,
    pub phase2_min_trading_days: Option<i32>,
    pub phase2_min_opened_positions: Option<i32>,
    pub phase2_revenue_share: Option<i32>,
    pub phase2_refund: Option<i32>,
    pub phase2_attempts: Option<i32>,

    #[serde(default)]
    pub phase3_daily_drawdown: Option<i32>,
    #[serde(default)]
    pub phase3_overall_drawdown: Option<i32>,
    #[serde(default)]
    pub phase3_target_profit: Option<i32>,
    #[serde(default)]
    pub phase3_duration: Option<i32>,
    #[serde(default)]
    pub phase3_min_trading_days: Option<i32>,
    #[serde(default)]
    pub phase3_min_opened_positions: Option<i32>,
    #[serde(default)]
    pub phase3_revenue_share: Option<i32>,
    #[serde(default)]
    pub phase3_refund: Option<i32>,
    #[serde(default)]
    pub phase3_attempts: Option<i32>,

    pub daily_drawdown: Option<i32>,
    pub overall_drawdown: Option<i32>,
    pub target_profit: Option<i32>,
    pub revenue_share: Option<i32>,

    #[serde(default = "default_refund")]
    pub refund: Option<i32>,

    #[serde(default)]
    pub trader_package_type: Option<i32>,
}

fn default_refund() -> Option<i32> {
    Some(5)
}

impl TraderPackageFieldsOrderMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_package_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_package_id.into()
    }
}
