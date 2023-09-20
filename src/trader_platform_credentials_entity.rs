use crate::broker_no_sql::BrokerMyNoSql;
use crate::trading_platform_no_sql::TradingPlatformMyNoSql;
use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("trading-platform-credentials")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TraderPlatformCredentialsNoSqlEntity {
    pub broker: BrokerMyNoSql,

    pub trading_platform: TradingPlatformMyNoSql,

    pub windows: String,

    pub mac: String,

    pub ios: String,

    pub android: String,

    pub web: String,
}

impl TraderPlatformCredentialsNoSqlEntity {
    pub fn generate_partition_key(broker: BrokerMyNoSql) -> &'static str {
        match broker {
            BrokerMyNoSql::WelltradeDemo => "welltrade-demo",
            BrokerMyNoSql::WelltradeLive => "welltrade-live",
        }
    }

    pub fn generate_row_key<'s>(trading_platform: TradingPlatformMyNoSql) -> StrOrString<'s> {
        match trading_platform {
            TradingPlatformMyNoSql::MetaTrader4 => "mt4".into(),
            TradingPlatformMyNoSql::MetaTrader5 => "mt5".into(),
        }
    }
}
