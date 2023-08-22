use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum BrokerMyNoSql {
    WelltradeDemo = 0,
    WelltradeLive = 1,
}