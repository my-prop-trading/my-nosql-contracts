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
        payout_type.into()
    }

    pub fn generate_row_key(min_max_type: MinMaxType) -> &'static str {
        min_max_type.into()
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

// Also create PayoutType from string
impl PayoutType {
    fn from_str(payout_type: &str) -> Self {
        match payout_type {
            "bank-transfer" => PayoutType::BankTransfer,
            "crypto" => PayoutType::Crypto,
            _ => PayoutType::BankTransfer,
        }
    }
}

impl From<PayoutType> for &'static str {
    fn from(payout_type: PayoutType) -> Self {
        match payout_type {
            PayoutType::BankTransfer => "bank-transfer",
            PayoutType::Crypto => "crypto",
        }
    }
}


// Also create MinMaxType from string
impl MinMaxType {
    fn from_str(min_max_type: &str) -> Self {
        match min_max_type {
            "min" => MinMaxType::Min,
            "max" => MinMaxType::Max,
            _ => MinMaxType::Min,
        }
    }
}

impl From<MinMaxType> for &'static str {
    fn from(min_max_type: MinMaxType) -> Self {
        match min_max_type {
            MinMaxType::Min => "min",
            MinMaxType::Max => "max",
        }
    }
}

