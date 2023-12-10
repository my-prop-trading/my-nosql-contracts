use std::str::FromStr;
use serde::*;

pub const PLATFORM_METATRADER_4: &str = "MetaTrader4";
pub const PLATFORM_METATRADER_5: &str = "MetaTrader5";

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(i32)]
pub enum TradingPlatformMyNoSql {
    MetaTrader4 = 0,
    MetaTrader5 = 1,
}

impl TradingPlatformMyNoSql {
    pub fn as_str(&self) -> &'static str {
        match self {
            TradingPlatformMyNoSql::MetaTrader4 => PLATFORM_METATRADER_4,
            TradingPlatformMyNoSql::MetaTrader5 => PLATFORM_METATRADER_5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradingPlatformParseError {
    pub error: String,
}

impl std::fmt::Display for TradingPlatformParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse TradingPlatformMyNoSql from '{}' should be '{}(0)' or '{}(1)'",
            self.error, PLATFORM_METATRADER_4, PLATFORM_METATRADER_5
        )
    }
}

impl std::error::Error for TradingPlatformParseError {}

impl FromStr for TradingPlatformMyNoSql {
    type Err = TradingPlatformParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = s.to_string();
        match s {
            PLATFORM_METATRADER_4 => Ok(TradingPlatformMyNoSql::MetaTrader4),
            PLATFORM_METATRADER_5 => Ok(TradingPlatformMyNoSql::MetaTrader5),
            _ => Err(TradingPlatformParseError { error }),
        }
    }

}

impl From<TradingPlatformMyNoSql> for i32 {
    fn from(value: TradingPlatformMyNoSql) -> Self {
        match value {
            TradingPlatformMyNoSql::MetaTrader4 => 0,
            TradingPlatformMyNoSql::MetaTrader5 => 1,
        }
    }
}

impl From<i32> for TradingPlatformMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => TradingPlatformMyNoSql::MetaTrader4,
            1 => TradingPlatformMyNoSql::MetaTrader5,
            _ => panic!(
                "Invalid value '{}' for TradingPlatformMyNoSql should be '0 - {}' or '1 - {}'",
                value, PLATFORM_METATRADER_4, PLATFORM_METATRADER_5
            )
        }
    }
}