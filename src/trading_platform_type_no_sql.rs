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

    pub fn from_str(
        trading_platform_type: &'static str,
    ) -> TradingPlatformTypeMyNoSql {
        match trading_platform_type {
            PLATFORM_DEMO => TradingPlatformTypeMyNoSql::Demo,
            PLATFORM_LIVE => TradingPlatformTypeMyNoSql::Live,
            _ => panic!(
                "TradingPlatformType should be '{}' or '{}'",
                PLATFORM_DEMO, PLATFORM_LIVE
            ),
        }
    }
}