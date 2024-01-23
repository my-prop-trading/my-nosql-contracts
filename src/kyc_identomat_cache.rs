use service_sdk::rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

pub const PROOF_OF_IDENTITY: &str = "ProofOfIdentity";
pub const PROOF_OF_ADDRESS: &str = "ProofOfAddress";

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(i32)]
pub enum KycIdentomatStatusMyNoSql {
    Created = 0,
    Updated = 1,
    Deleted = 2,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum KycProofTypeMyNoSql {
    ProofOfIdentity = 0,
    ProofOfAddress = 1,
}

impl KycProofTypeMyNoSql {
    pub fn as_str(&self) -> &'static str {
        match self {
            KycProofTypeMyNoSql::ProofOfIdentity => PROOF_OF_IDENTITY,
            KycProofTypeMyNoSql::ProofOfAddress => PROOF_OF_ADDRESS,
        }
    }
}

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
