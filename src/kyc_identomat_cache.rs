use service_sdk::rust_extensions::StrOrString;
use serde::*;
use crate::kyc_proof_type_no_sql::KycProofTypeMyNoSql;
service_sdk::macros::use_my_no_sql_entity!();

pub const PROOF_OF_IDENTITY: &str = "ProofOfIdentity";
pub const PROOF_OF_ADDRESS: &str = "ProofOfAddress";

#[my_no_sql_entity("kyc-identomat-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KycIdentomatCacheMyNoSqlEntity {
    pub applicant_id: String,
    pub status: KycIdentomatStatusMyNoSql,
}

impl KycIdentomatCacheMyNoSqlEntity {

    pub fn generate_partition_key<'s>(client_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        client_id.into()
    }

    pub fn generate_row_key(kyc_proof_type: KycProofTypeMyNoSql) -> &'static str {
        kyc_proof_type.as_str()
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum KycIdentomatStatusMyNoSql {
    Created = 0,
    Updated = 1,
    Deleted = 2,
}