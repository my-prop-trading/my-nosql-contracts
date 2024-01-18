use std::collections::HashMap;


use serde::*;

use crate::EmailTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(i32)]
pub enum KycProviderTypeMyNoSql {
    Sumsub = 0,
    Identomat = 1,
}

#[enum_model(partition_key:"settings", row_key: "kyc")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KycSettingsModel {
    pub provider: KycProviderTypeMyNoSql,
}