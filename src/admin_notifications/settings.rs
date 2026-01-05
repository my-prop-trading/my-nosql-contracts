use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("admin-notifications-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AdminNotificationSettingsMyNoSqlEntity {
    #[serde(default)]
    pub channels: Vec<AdminNotificationChannelMyNosqlModel>,
    pub card_multiple_geo: Option<CardMultipleGeoMyNosqlModel>,
    pub card_shared_name: Option<CardSharedNameMyNosqlModel>,
    pub fail_suspicious_pattern: Option<FailSuspiciousPatternMyNosqlModel>,
    pub excessive_active_accounts: Option<ExcessiveActiveAccountsMyNosqlModel>,
    pub large_account_size: Option<LargeAccountSizeMyNosqlModel>,
    pub fast_success_payin_sequence: Option<FastSuccessPayinSequenceMyNosqlModel>,
    pub exessive_success_payins: Option<ExessiveSuccessPayinsMyNosqlModel>,
    pub fast_affiliate_payins: Option<FastAffiliatePayinsMyNosqlModel>,
    pub multiaccount: Option<MultiaccountSettingsMyNosqlModel>,
    pub new_payout: Option<NewPayoutSettingsMyNosqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AdminNotificationChannelMyNosqlModel {
    pub slack_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CardMultipleGeoMyNosqlModel {
    pub channel_id: String,
    pub period_minutes: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardSharedNameMyNosqlModel {
    pub channel_id: String,
    pub clients_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FailSuspiciousPatternMyNosqlModel {
    pub channel_id: String,
    pub period_minutes: i32,
    pub payins_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExcessiveActiveAccountsMyNosqlModel {
    pub channel_id: String,
    pub accounts_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LargeAccountSizeMyNosqlModel {
    pub channel_id: String,
    pub biggest_account_size: f64,
    pub no_discount_account_size: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FastSuccessPayinSequenceMyNosqlModel {
    pub channel_id: String,
    pub payins_count: i32,
    pub interval_minutes: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExessiveSuccessPayinsMyNosqlModel {
    pub channel_id: String,
    pub payins_count: i32,
    pub interval_minutes: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FastAffiliatePayinsMyNosqlModel {
    pub channel_id: String,
    pub interval_minutes: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MultiaccountSettingsMyNosqlModel {
    pub channel_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NewPayoutSettingsMyNosqlModel {
    pub channel_id: String,
}

impl AdminNotificationSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "*"
    }

    pub fn generate_row_key() -> &'static str {
        "*"
    }
}
