use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("payout-withdrawal-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayoutWithdrawalSettingsMyNoSqlEntity {
    pub value: f64,
    pub currency: String,
}

impl PayoutWithdrawalSettingsMyNoSqlEntity {
    pub fn generate_partition_key(payout_type: PayoutType) -> &'static str {
        match payout_type {
            PayoutType::BankTransfer => "bank-transfer",
            PayoutType::Crypto => "crypto",
        }
    }

    pub fn generate_row_key(min_max_type: MinMaxType) -> &'static str {
        match min_max_type {
            MinMaxType::Min => "min".into(),
            MinMaxType::Max => "max".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PayoutType {
    BankTransfer = 0,
    Crypto = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MinMaxType {
    Min = 0,
    Max = 1,
}