use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("admin-notification-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AdminNotificationSettingsMyNoSqlEntity {
    pub category: Vec<AdminNotificationCategoryMyNosqlModel>,
    pub large_account_size: Option<LargeAccountSizeMyNosqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminNotificationCategoryMyNosqlModel {
    pub slack_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LargeAccountSizeMyNosqlModel {
    pub category_name: String,
    pub account_size: f64,
    pub no_discount_price: f64,
}

impl AdminNotificationSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "*"
    }

    pub fn generate_row_key() -> &'static str {
        "*"
    }
}
