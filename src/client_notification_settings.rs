use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

/// Whether clients see soft-breach notifications on the dashboard, per rule.
/// Breaches are counted regardless; this only gates what the client is shown.
#[my_no_sql_entity("client-notification-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ClientNotificationSettingsMyNoSqlEntity {
    pub exposure_loss_soft_breach: ClientNotificationRuleMyNoSql,
    pub margin_rule_soft_breach: ClientNotificationRuleMyNoSql,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ClientNotificationRuleMyNoSql {
    pub visible_to_clients: bool,
    /// How long the notification stays after the last breach tick; 0 = only while breaching.
    pub visibility_minutes: u32,
}

impl ClientNotificationSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "*"
    }

    pub fn generate_row_key() -> &'static str {
        "settings"
    }
}
