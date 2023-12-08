use serde::{Deserialize, Serialize};

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