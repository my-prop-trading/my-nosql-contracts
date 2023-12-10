use std::str::FromStr;
use serde::*;

pub const PLATFORM_DEMO: &str = "Demo";
pub const PLATFORM_LIVE: &str = "Live";

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum TradingPlatformTypeMyNoSql {
    Demo = 0,
    Live = 1,
}

impl TradingPlatformTypeMyNoSql {
    pub fn as_str(&self) -> &'static str {
        match self {
            TradingPlatformTypeMyNoSql::Demo => PLATFORM_DEMO,
            TradingPlatformTypeMyNoSql::Live => PLATFORM_LIVE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradingPlatformTypeParseError {
    pub error: String,
}

impl std::fmt::Display for TradingPlatformTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse TradingPlatformTypeMyNoSql from '{}' should be '{}' or '{}'",
            self.error, PLATFORM_DEMO, PLATFORM_LIVE
        )
    }
}

impl std::error::Error for TradingPlatformTypeParseError {}

impl FromStr for TradingPlatformTypeMyNoSql {
    type Err = TradingPlatformTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = s.to_string();
        match s {
            PLATFORM_DEMO => Ok(TradingPlatformTypeMyNoSql::Demo),
            PLATFORM_LIVE => Ok(TradingPlatformTypeMyNoSql::Live),
            _ => Err(TradingPlatformTypeParseError { error }),
        }
    }
}

impl From<TradingPlatformTypeMyNoSql> for i32 {
    fn from(value: TradingPlatformTypeMyNoSql) -> Self {
        match value {
            TradingPlatformTypeMyNoSql::Demo => 0,
            TradingPlatformTypeMyNoSql::Live => 1,
        }
    }
}

impl From<i32> for TradingPlatformTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => TradingPlatformTypeMyNoSql::Demo,
            1 => TradingPlatformTypeMyNoSql::Live,
            _ => panic!(
                "Invalid value '{}' for TradingPlatformTypeMyNoSql should be '{}' or '{}'",
                value, PLATFORM_DEMO, PLATFORM_LIVE
            )
        }
    }
}

