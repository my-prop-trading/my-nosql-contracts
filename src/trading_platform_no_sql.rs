use serde::{Deserialize, Serialize};

pub const PLATFORM_METATRADER_4: &str = "MetaTrader4";
pub const PLATFORM_METATRADER_5: &str = "MetaTrader5";

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(i32)]
pub enum TradingPlatformMyNoSql {
    MetaTrader4 = 0,
    MetaTrader5 = 1,
}