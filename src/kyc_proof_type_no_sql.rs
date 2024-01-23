use std::str::FromStr;
use serde::*;

pub const PROOF_OF_IDENTITY: &str = "ProofOfIdentity";
pub const PROOF_OF_ADDRESS: &str = "ProofOfAddress";

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

#[derive(Debug, Clone)]
pub struct KycProofTypeParseError {
    pub error: String,
}

impl std::fmt::Display for KycProofTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse KycProofTypeMyNoSql from '{}' should be '{}(0)' or '{}(1)'",
            self.error, PROOF_OF_IDENTITY, PROOF_OF_ADDRESS
        )
    }
}

impl std::error::Error for KycProofTypeParseError {}

impl FromStr for KycProofTypeMyNoSql {
    type Err = KycProofTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = s.to_string();
        match s {
            PROOF_OF_IDENTITY => Ok(KycProofTypeMyNoSql::ProofOfIdentity),
            PROOF_OF_ADDRESS => Ok(KycProofTypeMyNoSql::ProofOfAddress),
            _ => Err(KycProofTypeParseError { error }),
        }
    }
}

impl From<KycProofTypeMyNoSql> for i32 {
    fn from(value: KycProofTypeMyNoSql) -> Self {
        match value {
            KycProofTypeMyNoSql::ProofOfIdentity => 0,
            KycProofTypeMyNoSql::ProofOfAddress => 1,
        }
    }
}

impl From<i32> for KycProofTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => KycProofTypeMyNoSql::ProofOfIdentity,
            1 => KycProofTypeMyNoSql::ProofOfAddress,
            _ => panic!(
                "Invalid value '{}' for KycProofTypeMyNoSql should be '{}(0)' or '{}(1)'",
                value, PROOF_OF_IDENTITY, PROOF_OF_ADDRESS
            )
        }
    }
}

