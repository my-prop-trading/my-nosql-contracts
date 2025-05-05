use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("payment-pages")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentPageMyNoSqlEntity {
    pub expires: Timestamp,
    pub html: String,
    pub order_id: String,
    pub client_id: String,
}

impl PaymentPageMyNoSqlEntity {
    pub fn generate_partition_key() -> String {
        "*".into()
    }

    pub fn generate_row_key(order_id: impl Into<String>) -> String {
        order_id.into()
    }
}
