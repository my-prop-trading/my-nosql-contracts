use serde::*;
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

service_sdk::macros::use_my_no_sql_entity!();

const RFC3339_DATE_LEN: usize = "YYYY-MM-DD".len();
const SETTINGS_VERSIONS_PARTITION_KEY: &str = "versions";
const SETTINGS_ROW_KEY_WIDTH: usize = 20;

/// A news event the rule watches: active and released at an exact time. Partitioned by the UTC
/// day of release.
#[my_no_sql_entity("news-events")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NewsEventMyNoSqlEntity {
    pub title: String,
    // ISO 3166-1 alpha-2; EU for the euro area.
    pub country: String,
    // ISO 4217.
    pub currency: String,
    // news_trading.ImpactType value.
    pub impact: i32,
    // UTC, unix microseconds.
    pub release_time: i64,
}

impl NewsEventMyNoSqlEntity {
    /// Returns the UTC day of a moment as `YYYY-MM-DD`.
    pub fn generate_partition_key(unix_microseconds: i64) -> String {
        let mut day = DateTimeAsMicroseconds::new(unix_microseconds).to_rfc3339();
        day.truncate(RFC3339_DATE_LEN);
        day
    }

    pub fn generate_row_key(event_id: &str) -> &str {
        event_id
    }
}

/// The news an instrument is exposed to: events whose currency is in `currencies` or whose
/// country is in `countries`. An instrument without a row is exposed to none.
#[my_no_sql_entity("news-instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NewsInstrumentMyNoSqlEntity {
    pub currencies: Vec<String>,
    pub countries: Vec<String>,
}

impl NewsInstrumentMyNoSqlEntity {
    pub fn generate_partition_key(platform_id: i32) -> String {
        platform_id.to_string()
    }

    pub fn generate_row_key(symbol: &str) -> &str {
        symbol
    }
}

/// One version of the news rule settings. The version in force at a moment is the latest whose
/// `effective_from` is at or before it; without any version the rule is off.
#[my_no_sql_entity("news-rule-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NewsRuleSettingsMyNoSqlEntity {
    // UTC, unix microseconds.
    pub effective_from: i64,
    pub enabled: bool,
    pub before_minutes: i32,
    pub after_minutes: i32,
    // news_trading.AccountTypeRestrictions value.
    pub account_scope: i32,
    pub in_window_deduction_percent: f64,
    pub long_hold_minutes: i32,
    pub long_hold_deduction_percent: f64,
    // news_trading.ImpactType values.
    pub impacts: Vec<i32>,
}

impl NewsRuleSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        SETTINGS_VERSIONS_PARTITION_KEY
    }

    /// Zero-padded, so row keys sort in time order.
    pub fn generate_row_key(effective_from: i64) -> String {
        format!("{effective_from:0SETTINGS_ROW_KEY_WIDTH$}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_partition_is_utc_day() {
        let last_second_of_day = 1_794_009_599_000_000;
        let midnight = 1_794_009_600_000_000;

        assert_eq!(
            "2026-11-06",
            NewsEventMyNoSqlEntity::generate_partition_key(last_second_of_day)
        );
        assert_eq!(
            "2026-11-07",
            NewsEventMyNoSqlEntity::generate_partition_key(midnight)
        );
    }

    #[test]
    fn settings_row_keys_sort_in_time_order() {
        let earlier = NewsRuleSettingsMyNoSqlEntity::generate_row_key(999_999_999_999_999);
        let later = NewsRuleSettingsMyNoSqlEntity::generate_row_key(1_793_971_800_000_000);
        assert!(earlier < later);
    }
}
