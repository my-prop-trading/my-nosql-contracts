use service_sdk::rust_extensions::StrOrString;
use crate::kyc_proof_type_no_sql::KycProofTypeMyNoSql;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("identomat-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IdentomatCacheMyNoSqlEntity {
    pub applicant_id: String,
}

impl IdentomatCacheMyNoSqlEntity {

    pub fn generate_partition_key<'s>(client_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        client_id.into()
    }

    pub fn generate_row_key(kyc_proof_type: KycProofTypeMyNoSql) -> &'static str {
        kyc_proof_type.as_str()
    }
}
