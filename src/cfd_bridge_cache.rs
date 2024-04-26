use service_sdk::rust_extensions::StrOrString;
use serde::*;
use crate::kyc_proof_type_no_sql::KycProofTypeMyNoSql;
service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum AccountTypeMyNoSql {
    Demo = 0,
    Live = 1,
}

#[my_no_sql_entity("cfd-bridge-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CfdBridgeCacheMyNoSqlEntity {
    pub trader_account_id: String,
    pub platform_account_id: String,
    pub balance: f64,
    pub equity: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
    pub account_type: AccountTypeMyNoSql,
}

impl CfdBridgeCacheMyNoSqlEntity {
    pub fn generate_partition_key_demo() -> &'static str {
        AccountTypeMyNoSql::Demo.as_str()
    }

    pub fn generate_partition_key_live() -> &'static str {
        AccountTypeMyNoSql::Live.as_str()
    }

    pub fn generate_partition_key<'s>(account_type: i32) -> StrOrString<'s> {
        AccountTypeMyNoSql::from(account_type).to_string().into()
    }

    pub fn generate_row_key<'s>(trader_account_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_account_id.into()
    }
}

impl AccountTypeMyNoSql {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Demo => "Demo",
            Self::Live => "live",
        }
    }
}

impl ToString for AccountTypeMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for AccountTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => AccountTypeMyNoSql::Demo,
            1 => AccountTypeMyNoSql::Live,
            _ => panic!(
                "Invalid value '{}' for AccountTypeMyNoSql",
                value,
            ),
        }
    }
}
