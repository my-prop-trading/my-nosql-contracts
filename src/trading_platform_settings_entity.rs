use crate::trading_broker_slot_no_sql::TradingBrokerSlotMyNoSql;
use crate::trading_platform_no_sql::TradingPlatformMyNoSql;
use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trading-platform-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPlatformSettingsNoSqlEntity {
    brand_settings: BrandSettings,
    live_account_settings: Option<LiveAccountSettings>,
    tech_settings: TechSettings,
}

impl TradingPlatformSettingsNoSqlEntity {
    pub fn generate_partition_key(trading_platform_type: TradingPlatformMyNoSql) -> &'static str {
        match trading_platform_type {
            TradingPlatformMyNoSql::MetaTrader4 => "mt4".into(),
            TradingPlatformMyNoSql::MetaTrader5 => "mt5".into(),
        }
    }

    pub fn generate_row_key<'s>(trading_platform_slot: TradingBrokerSlotMyNoSql) -> StrOrString<'s> {
        match trading_platform_slot {
            TradingBrokerSlotMyNoSql::Slot0Demo => "0".into(),
            TradingBrokerSlotMyNoSql::Slot0Live => "1".into(),
            TradingBrokerSlotMyNoSql::Slot1Demo => "2".into(),
            TradingBrokerSlotMyNoSql::Slot1Live => "3".into(),
            TradingBrokerSlotMyNoSql::Slot2Demo => "4".into(),
            TradingBrokerSlotMyNoSql::Slot2Live => "5".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct BrandSettings {
    broker: Broker,
    links: Links,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Broker {
    name: String,
    r#type: String,
    compatible_name: String,
    caption: String,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Links {
    windows: String,
    mac: String,
    ios: String,
    android: String,
    web: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct LiveAccountSettings {
    partition_key: String,
    row_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct ServiceBus {
    url: String,
    topic: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct TechSettings {
    listening_port: String,
    server: String,
    manager_login: String,
    password: String,
    reconnect_timeout: i32,
    service_bus: ServiceBus,
    default_group: String,
    archive_group: String,
    accounts_ranges: AccountsRanges,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct AccountsRanges {
    use_range: bool,
    min: i64,
    max: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_config() {
    // Your JSON data here
    let json_data = r#"
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt4",
            "RowKey": "0",
            "BrandSettings": {
                "Broker": {
                    "Name": "Welltrade",
                    "Type": "Demo",
                    "CompatibleName": "WelltradeDemo",
                    "Caption": "MT4 Welltrade Super Caption",
                    "Enabled": true
                },
                "Links": {
                    "Windows": "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe",
                    "Mac": "https://download.mql5.com/cdn/web/metaquotes.software.corp/mt5/MetaTrader5.dmg",
                    "Ios": "https://download.mql5.com/cdn/mobile/mt5/ios?hl=en&server=Weltrade-Live,Weltrade-Demo=8",
                    "Android": "https://download.mql5.com/cdn/mobile/mt5/android?hl=en&server=Weltrade-Live,Weltrade-Demo",
                    "Web": "https://www.weltrade.com/webterminal/?lang=en&version=5"
                }
            },
            "LiveAccountSettings": {
                "PartitionKey": "mt4",
                "RowKey": "1"
            },
            "TechSettings": {
                "ListeningPort": "5000",
                "Server": "***",
                "ManagerLogin": "***",
                "Password": "***",
                "ReconnectTimeout": 15,
                "ServiceBus": {
                    "Url": "....",
                    "Topic": "...."
                },
                "DefaultGroup": "demo",
                "ArchiveGroup": "demo_disabled",
                "AccountsRanges": {
                    "UseRange": true,
                    "Min": 2700000,
                    "Max": 2800000
                }
            }
        }
    "#;

    let parsed_config: TradingPlatformSettingsNoSqlEntity = 
        serde_json::from_str(json_data)
        .unwrap();
    assert_eq!(
        parsed_config.brand_settings.links.windows, 
        "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe".to_string());
}
}
