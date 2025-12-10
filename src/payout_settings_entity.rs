use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("payout-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayoutSettingsMyNoSqlEntity {
    pub currencies: Vec<String>,
    pub cryptos: Vec<CryptoCurrencyMyNoSqlModel>,
    pub is_bank_transfer_enabled: bool,
    pub bank_transfer_limits: PayoutLimitsSettingsMyNoSqlModel,
    pub crypto_limits: PayoutLimitsSettingsMyNoSqlModel,
    pub excessive_profit_percent: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayoutLimitsSettingsMyNoSqlModel {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_currency: String,
    pub max_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CryptoCurrencyMyNoSqlModel {
    pub crypto: String,
    pub networks: Vec<String>,
}

impl PayoutSettingsMyNoSqlEntity {
    pub fn default_pk() -> String {
        "*".into()
    }

    pub fn default_rk() -> String {
        "*".into()
    }

    pub fn trading_package_rk(trading_package_id: impl Into<String>) -> String {
        trading_package_id.into()
    }
}
