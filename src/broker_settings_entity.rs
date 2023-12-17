use rust_extensions::StrOrString;
use serde::*;

use crate::{TradingPlatformTypeMyNoSql, TradingPlatformMyNoSql};
use crate::{PLATFORM_METATRADER_4, PLATFORM_METATRADER_5};

pub const BROKER_SETTINGS: &str = "Broker";

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("broker-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BrokerSettingsNoSqlEntity {
    #[serde(rename = "Id")]
    id: i32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Enabled")]
    enabled: bool,
    #[serde(rename = "IconUrl")]
    icon_url: String,
    #[serde(rename = "MetaTrader4")]
    meta_trader4: Platform,
    #[serde(rename = "MetaTrader5")]
    meta_trader5: Platform,
}

impl BrokerSettingsNoSqlEntity {
    pub fn generate_partition_key<'s>() -> StrOrString<'s> {
        BROKER_SETTINGS.into()
    }

    pub fn generate_row_key<'s>(broker_slot_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        broker_slot_id.into()
    }

    pub fn get_broker_id(&self) -> i32 {
        match self.row_key.parse() {
            Ok(num) => num,
            Err(err) => panic!("Failed to parse broker id: {:?}", err),
        }        
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Platform {
    enabled: bool,
    platform: TradingPlatformMyNoSql,
    links: Links,
    demo: PlatformSettings,
    live: PlatformSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct PlatformSettings {
    name: String,
    id: i32,
    r#type: TradingPlatformTypeMyNoSql,
    tech_settings: TechSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Links {
    pub windows: String,
    pub mac: String,
    pub ios: String,
    pub android: String,
    pub web: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TechSettings {
    pub server: String,
    pub manager_login: String,
    pub password: String,
    pub reconnect_timeout: u32,
    pub default_group: String,
    pub archive_group: String,
    pub accounts_ranges: AccountsRanges,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountsRanges {
    pub use_range: bool,
    pub min: u64,
    pub max: u64,
}

#[cfg(test)]
mod tests {
    use crate::{PLATFORM_DEMO, PLATFORM_LIVE};
    use crate::{PLATFORM_METATRADER_4, PLATFORM_METATRADER_5};

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_metatrader_short_config() {
        // Your JSON data here
        let json_data = r#"
        {
            "TimeStamp": "2023-09-02T07:59:40.8484",
            "PartitionKey": "Broker",
            "RowKey": "0",
            "Id": 0,
            "Name": "Weltrade",
            "Enabled": true,
            "IconUrl": "",
            "MetaTrader4": {
                "Enabled": true,
                "Platform": "MetaTrader4",
                "Links": {
                    "Windows": "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe",
                    "Mac": "https://download.mql5.com/cdn/web/metaquotes.software.corp/mt5/MetaTrader5.dmg",
                    "Ios": "https://download.mql5.com/cdn/mobile/mt5/ios?hl=en&server=Weltrade-Live,Weltrade-Demo=8",
                    "Android": "https://download.mql5.com/cdn/mobile/mt5/android?hl=en&server=Weltrade-Live,Weltrade-Demo",
                    "Web": "https://www.weltrade.com/webterminal/?lang=en&version=5"
                },
                "Demo": {
                    "Id": 0,
                    "Name": "WeltradeDemo",
                    "Type": "Demo",
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
                "Live": {
                    "Id": 1,
                    "Name": "WeltradLive",
                    "Type": "Live",
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
            },
            "MetaTrader5": {
                "Enabled": true,
                "Platform": "MetaTrader5",
                "Links": {
                    "Windows": "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe",
                    "Mac": "https://download.mql5.com/cdn/web/metaquotes.software.corp/mt5/MetaTrader5.dmg",
                    "Ios": "https://download.mql5.com/cdn/mobile/mt5/ios?hl=en&server=Weltrade-Live,Weltrade-Demo=8",
                    "Android": "https://download.mql5.com/cdn/mobile/mt5/android?hl=en&server=Weltrade-Live,Weltrade-Demo",
                    "Web": "https://www.weltrade.com/webterminal/?lang=en&version=5"
                },
                "Demo": {
                    "Id": 0,
                    "Name": "WeltradeDemo",
                    "Type": "Demo",
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
                "Live": {
                    "Id": 1,
                    "Name": "WeltradeLive",
                    "Type": "Live",
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
            }
        }
    "#;

        let parsed_config: BrokerSettingsNoSqlEntity =
            serde_json::from_str(json_data).unwrap();
        assert_eq!(
            parsed_config.meta_trader4.links.windows,
            "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe"
                .to_string()
        );
    }

    #[test]
    fn test_deserialize_default_weltrade_config() {
        // Your JSON data here
        let json_data = r#"
        [
            {
                "TimeStamp": "2023-09-02T07:59:40.8484",
                "PartitionKey": "Broker",
                "RowKey": "0",
                "Id": 0,
                "Name": "Weltrade",
                "Enabled": true,
                "IconUrl": "",
                "MetaTrader4": {
                    "Enabled": true,
                    "Platform": "MetaTrader4",
                    "Links": {
                        "Windows": "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe",
                        "Mac": "https://download.mql5.com/cdn/web/metaquotes.software.corp/mt5/MetaTrader5.dmg",
                        "Ios": "https://download.mql5.com/cdn/mobile/mt5/ios?hl=en&server=Weltrade-Live,Weltrade-Demo=8",
                        "Android": "https://download.mql5.com/cdn/mobile/mt5/android?hl=en&server=Weltrade-Live,Weltrade-Demo",
                        "Web": "https://www.weltrade.com/webterminal/?lang=en&version=5"
                    },
                    "Demo": {
                        "Name": "WeltradeDemo",
                        "Id": 0,
                        "Type": "Demo",
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
                    "Live": {
                        "Id": 1,
                        "Type": "Live",
                        "Name": "WeltradeLive",
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
                },
                "MetaTrader5": {
                    "Enabled": true,
                    "Platform": "MetaTrader5",
                    "Links": {
                        "Windows": "https://download.mql5.com/cdn/web/systemgates.limited/mt5/weltrade5setup.exe",
                        "Mac": "https://download.mql5.com/cdn/web/metaquotes.software.corp/mt5/MetaTrader5.dmg",
                        "Ios": "https://download.mql5.com/cdn/mobile/mt5/ios?hl=en&server=Weltrade-Live,Weltrade-Demo=8",
                        "Android": "https://download.mql5.com/cdn/mobile/mt5/android?hl=en&server=Weltrade-Live,Weltrade-Demo",
                        "Web": "https://www.weltrade.com/webterminal/?lang=en&version=5"
                    },
                    "Demo": {
                        "Name": "WeltradeDemo",
                        "Id": 0,
                        "Type": "Demo",
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
                    "Live": {
                        "Id": 1,
                        "Type": "Live",
                        "Name": "WeltradeLive",
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
                }
            },
            {
                "TimeStamp": "2023-09-02T07:59:40.8484",
                "PartitionKey": "Broker",
                "RowKey": "1",
                "Id": 1,
                "Name": "M4Markets",
                "Enabled": true,
                "IconUrl": "",
                "MetaTrader4": {
                    "Enabled": false,
                    "Platform": "MetaTrader4",
                    "Links": {
                        "Windows": "",
                        "Mac": "",
                        "Ios": "",
                        "Android": "",
                        "Web": ""
                    },
                    "Demo": {
                        "Name": "M4MarketsDemo",
                        "Id": 2,
                        "Type": "Demo",
                        "TechSettings": {
                            "Server": "",
                            "ManagerLogin": "",
                            "Password": "",
                            "ReconnectTimeout": 0,
                            "DefaultGroup": "",
                            "ArchiveGroup": "",
                            "AccountsRanges": {
                                "UseRange": false,
                                "Min": 0,
                                "Max": 0
                            }
                        }
                    },
                    "Live": {
                        "Id": 3,
                        "Name": "M4MarketsLive",
                        "Type": "Live",
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
                },
                "MetaTrader5": {
                    "Enabled": true,
                    "Platform": "MetaTrader5",
                    "Links": {
                        "Windows": "",
                        "Mac": "",
                        "Ios": "",
                        "Android": "",
                        "Web": ""
                    },
                    "Demo": {
                        "Name": "M4MarketsDemo",
                        "Id": 2,
                        "Type": "Demo",
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
                    "Live": {
                        "Id": 3,
                        "Type": "Live",
                        "Name": "M4MarketsLive",
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
                }
            },
            {
                "TimeStamp": "2023-09-02T07:59:40.8484",
                "PartitionKey": "Broker",
                "RowKey": "2",
                "Id": 2,
                "Name": "",
                "Enabled": false,
                "IconUrl": "",
                "MetaTrader4": {
                    "Enabled": false,
                    "Platform": "MetaTrader4",
                    "Links": {
                        "Windows": "",
                        "Mac": "",
                        "Ios": "",
                        "Android": "",
                        "Web": ""
                    },
                    "Demo": {
                        "Name": "",
                        "Id": 4,
                        "Type": "Demo",
                        "TechSettings": {
                            "Server": "",
                            "ManagerLogin": "",
                            "Password": "",
                            "ReconnectTimeout": 0,
                            "DefaultGroup": "",
                            "ArchiveGroup": "",
                            "AccountsRanges": {
                                "UseRange": false,
                                "Min": 0,
                                "Max": 0
                            }
                        }
                    },
                    "Live": {
                        "Id": 5,
                        "Name": "",
                        "Type": "Live",
                        "TechSettings": {
                            "Server": "",
                            "ManagerLogin": "",
                            "Password": "***",
                            "ReconnectTimeout": 15,
                            "DefaultGroup": "",
                            "ArchiveGroup": "0",
                            "AccountsRanges": {
                                "UseRange": false,
                                "Min": 0,
                                "Max": 0
                            }
                        }
                    }
                },
                "MetaTrader5": {
                    "Enabled": true,
                    "Platform": "MetaTrader5",
                    "Links": {
                        "Windows": "",
                        "Mac": "",
                        "Ios": "",
                        "Android": "",
                        "Web": ""
                    },
                    "Demo": {
                        "Name": "",
                        "Id": 4,
                        "Type": "Demo",
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
                    "Live": {
                        "Id": 5,
                        "Type": "Live",
                        "Name": "",
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
                }
            }
        ]
        "#;

        let parsed_config: Vec<BrokerSettingsNoSqlEntity> =
            serde_json::from_str(json_data).unwrap();
 
        // Broker1 mt4 slot 2 demo
        let broker_slot_0 = &parsed_config[0];
        assert_eq!(broker_slot_0.meta_trader4.platform.as_str(), PLATFORM_METATRADER_4);
        assert_eq!(broker_slot_0.id, 0);
        assert_eq!(broker_slot_0.name, "Weltrade".to_string());
        assert_eq!(
            broker_slot_0.meta_trader4.demo.r#type,
            TradingPlatformTypeMyNoSql::from_str(PLATFORM_DEMO).unwrap()
        );

        let broker_slot_1 = &parsed_config[1];
        assert_eq!(broker_slot_1.meta_trader4.platform.as_str(), PLATFORM_METATRADER_4);
        assert_eq!(broker_slot_1.id, 1);
        assert_eq!(broker_slot_1.name, "M4Markets".to_string());
        assert_eq!(
            broker_slot_1.meta_trader4.demo.r#type,
            TradingPlatformTypeMyNoSql::from_str(PLATFORM_DEMO).unwrap()
        );

        let broker_slot_2 = &parsed_config[2];
        assert_eq!(broker_slot_2.meta_trader4.platform.as_str(), PLATFORM_METATRADER_4);
        assert_eq!(broker_slot_2.id, 2);
        assert_eq!(broker_slot_2.name, "".to_string());
        assert_eq!(
            broker_slot_2.meta_trader4.enabled, false);
    }
}
