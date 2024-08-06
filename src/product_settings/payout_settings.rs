use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"settings", row_key: "payout")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayoutSettingsModel {
    pub currencies: Vec<String>,
    pub cryptos: Vec<CryptoDescription>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CryptoDescription {
    pub crypto: String,
    pub networks: Vec<String>,
}