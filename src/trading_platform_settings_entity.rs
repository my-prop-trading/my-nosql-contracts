use crate::trading_platform_no_sql::TradingPlatformMyNoSql;
use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum TradingPlatformTypeMyNoSql {
    Demo = 0,
    Live = 1,
}

#[my_no_sql_entity("trading-platform-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingPlatformSettingsNoSqlEntity {
    brand_settings: BrandSettings,
    live_account_settings: Option<LiveAccountSettings>,
    tech_settings: TechSettings,
}

impl TradingPlatformSettingsNoSqlEntity {
    pub fn generate_partition_key(trading_platform_server: TradingPlatformMyNoSql) -> &'static str {
        match trading_platform_server {
            TradingPlatformMyNoSql::MetaTrader4 => "mt4".into(),
            TradingPlatformMyNoSql::MetaTrader5 => "mt5".into(),
        }
    }

    pub fn generate_row_key<'s>(trading_platform_slot_id: u32) -> StrOrString<'s> {
        StrOrString::create_as_string(trading_platform_slot_id.to_string())
    }
}

pub fn get_trading_platform_type(
    trading_platform_type: &'static str,
) -> TradingPlatformTypeMyNoSql {
    match trading_platform_type {
        "Demo" => TradingPlatformTypeMyNoSql::Demo,
        "Live" => TradingPlatformTypeMyNoSql::Live,
        _ => panic!("TradingPlatformType should be 'Demo' or 'Live'"),
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
    r#type: TradingPlatformTypeMyNoSql,
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
struct TechSettings {
    server: String,
    manager_login: String,
    password: String,
    reconnect_timeout: u32,
    default_group: String,
    archive_group: String,
    accounts_ranges: AccountsRanges,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct AccountsRanges {
    use_range: bool,
    min: u64,
    max: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metatrader_short_config() {
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
                "Server": "***",
                "ManagerLogin": "***",
                "Password": "***",
                "ReconnectTimeout": 15,
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
            serde_json::from_str(json_data).unwrap();
        assert_eq!(
            parsed_config.brand_settings.links.windows,
            "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe"
                .to_string()
        );
    }

    #[test]
    fn test_deserialize_default_welltrade_config() {
        // Your JSON data here
        let json_data = r#"
        [
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
    "PartitionKey": "mt4",
    "RowKey": "0",
            "BrandSettings": {
                "Broker": {
                    "Name": "Welltrade",
                    "Type": "Demo",
                    "CompatibleName": "WelltradeDemo",
                    "Caption": "MT4 Welltrade",
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
                "Server": "***",
                "ManagerLogin": "***",
                "Password": "***",
                "ReconnectTimeout": 15,
                "DefaultGroup": "demo",
                "ArchiveGroup": "demo_disabled",
                "AccountsRanges": {
                    "UseRange": true,
                    "Min": 2700000,
                    "Max": 2800000
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt4",
            "RowKey": "1",
            "BrandSettings": {
                "Broker": {
                    "Name": "Welltrade",
                    "Type": "Live",
                    "CompatibleName": "WelltradeLive",
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
            "TechSettings": {
                "Server": "***",
                "ManagerLogin": "***",
                "Password": "***",
                "ReconnectTimeout": 15,
                "DefaultGroup": "demo",
                "ArchiveGroup": "demo_disabled",
                "AccountsRanges": {
                    "UseRange": true,
                    "Min": 2700000,
                    "Max": 2800000
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt5",
            "RowKey": "0",
            "BrandSettings": {
                "Broker": {
                    "Name": "Welltrade",
                    "Type": "Demo",
                    "CompatibleName": "WelltradeDemo",
    "Caption": "MT5 Welltrade Super Caption",
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
                "Server": "***",
                "ManagerLogin": "***",
                "Password": "***",
                "ReconnectTimeout": 15,
                "DefaultGroup": "demo",
                "ArchiveGroup": "demo_disabled",
                "AccountsRanges": {
                    "UseRange": true,
                    "Min": 2700000,
                    "Max": 2800000
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt5",
            "RowKey": "1",
            "BrandSettings": {
                "Broker": {
                    "Name": "Welltrade",
                    "Type": "Live",
                    "CompatibleName": "WelltradeLive",
                    "Caption": "MT5 Welltrade Super Caption",
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
            "TechSettings": {
                "Server": "***",
                "ManagerLogin": "***",
                "Password": "***",
                "ReconnectTimeout": 15,
                "DefaultGroup": "demo",
                "ArchiveGroup": "demo_disabled",
                "AccountsRanges": {
                    "UseRange": true,
                    "Min": 2700000,
                    "Max": 2800000
                }
            }
        },

        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt4",
            "RowKey": "2",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Demo",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "LiveAccountSettings": {
    "PartitionKey": "mt4",
    "RowKey": "3"
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt4",
            "RowKey": "3",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Live",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt5",
            "RowKey": "2",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Demo",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "LiveAccountSettings": {
                "PartitionKey": "mt5",
                "RowKey": "3"
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt5",
            "RowKey": "3",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Live",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
    "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },

        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt4",
            "RowKey": "4",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Demo",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "LiveAccountSettings": {
                "PartitionKey": "mt4",
                "RowKey": "5"
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt4",
            "RowKey": "5",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Live",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt5",
            "RowKey": "4",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Demo",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "LiveAccountSettings": {
                "PartitionKey": "mt5",
                "RowKey": "5"
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        },
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "mt5",
            "RowKey": "5",
            "BrandSettings": {
                "Broker": {
                    "Name": "",
                    "Type": "Live",
                    "CompatibleName": "",
                    "Caption": "",
                    "Enabled": false
                },
                "Links": {
                    "Windows": "",
                    "Mac": "",
                    "Ios": "",
                    "Android": "",
                    "Web": ""
                }
            },
            "TechSettings": {
                "Server": "",
                "ManagerLogin": "",
                "Password": "",
                "ReconnectTimeout": 15,
                "DefaultGroup": "",
                "ArchiveGroup": "",
                "AccountsRanges": {
                    "UseRange": false,
                    "Min": 0,
                    "Max": 0
                }
            }
        }        
        ]
        "#;

        let parsed_config: Vec<TradingPlatformSettingsNoSqlEntity> =
            serde_json::from_str(json_data).unwrap();

        // Broker1 mt4 slot 2 demo
        assert_eq!(parsed_config[4].partition_key, "mt4".to_string());
        assert_eq!(parsed_config[4].row_key, "2".to_string());
        assert_eq!(parsed_config[4].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[4].brand_settings.broker.r#type,
            get_trading_platform_type("Demo")
        );
        // Broker1 mt4 slot 3 live
        assert_eq!(parsed_config[5].partition_key, "mt4".to_string());
        assert_eq!(parsed_config[5].row_key, "3".to_string());
        assert_eq!(parsed_config[5].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[5].brand_settings.broker.r#type,
            get_trading_platform_type("Live")
        );
        // Broker1 mt5 slot 2 demo
        assert_eq!(parsed_config[6].partition_key, "mt5".to_string());
        assert_eq!(parsed_config[6].row_key, "2".to_string());
        assert_eq!(parsed_config[6].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[6].brand_settings.broker.r#type,
            get_trading_platform_type("Demo")
        );
        // Broker1 mt5 slot 3 live
        assert_eq!(parsed_config[7].partition_key, "mt5".to_string());
        assert_eq!(parsed_config[7].row_key, "3".to_string());
        assert_eq!(parsed_config[7].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[7].brand_settings.broker.r#type,
            get_trading_platform_type("Live")
        );

        // Broker2 mt4 slot 4 demo
        assert_eq!(parsed_config[8].partition_key, "mt4".to_string());
        assert_eq!(parsed_config[8].row_key, "4".to_string());
        assert_eq!(parsed_config[8].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[8].brand_settings.broker.r#type,
            get_trading_platform_type("Demo")
        );
        // Broker2 mt4 slot 5 live
        assert_eq!(parsed_config[9].partition_key, "mt4".to_string());
        assert_eq!(parsed_config[9].row_key, "5".to_string());
        assert_eq!(parsed_config[9].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[9].brand_settings.broker.r#type,
            get_trading_platform_type("Live")
        );
        // Broker2 mt5 slot 4 demo
        assert_eq!(parsed_config[10].partition_key, "mt5".to_string());
        assert_eq!(parsed_config[10].row_key, "4".to_string());
        assert_eq!(parsed_config[10].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[10].brand_settings.broker.r#type,
            get_trading_platform_type("Demo")
        );
        // Broker2 mt5 slot 5 live
        assert_eq!(parsed_config[11].partition_key, "mt5".to_string());
        assert_eq!(parsed_config[11].row_key, "5".to_string());
        assert_eq!(parsed_config[11].brand_settings.broker.name, "".to_string());
        assert_eq!(
            parsed_config[11].brand_settings.broker.r#type,
            get_trading_platform_type("Live")
        );
    }
}
