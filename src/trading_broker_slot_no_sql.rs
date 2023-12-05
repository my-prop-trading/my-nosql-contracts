use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum TradingBrokerSlotMyNoSql {
    Slot0Demo = 0,
    Slot0Live = 1,
    Slot1Demo = 2,
    Slot1Live = 3,
    Slot2Demo = 4,
    Slot2Live = 5,
}