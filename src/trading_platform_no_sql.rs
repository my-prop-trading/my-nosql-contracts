use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(i32)]
pub enum TradingPlatformMyNoSql {
    MetaTrader4 = 0,
    MetaTrader5 = 1,
}