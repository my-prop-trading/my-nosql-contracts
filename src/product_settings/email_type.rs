#[derive(Debug)]
pub enum EmailType {
    RegistrationConfirmation,
    ForgotPassword,
    PasswordResetNotification,
    SuccessfulScenarioPhase1,
    SuccessfulScenarioPhase2,
    SuccessfulScenarioPhase2KycRequired,
    SuccessfulScenarioPhase2ContractRequired,
    SuccessfulProofOfIdentity,
    RetryProofOfIdentity,
    FailedProofOfIdentity,
    SuccessfulProofOfAddress,
    RetryProofOfAddress,
    FailedProofOfAddress,
    ContractToClient,
    ServiceDeniedDueToViolationTermsAndCondOnContractSigning,
    ServiceDeniedOnAllAccountsDueToViolationTermsAndCond,
    ServiceDeniedDueToViolationOfOurTermsAndCond,
    ServiceDeniedDueToViolationOfOurTermsAndCondFutureProfitBlocked,
    ServiceDeniedDueToViolationOfOurTermsAndCondFutureAndCurrentProfitBlocked,
    LiveAccountGranted,
    LiveAccountLost ,
    PaymentReceived,
    PaymentFailed,
    FailedScenarioPhase1,
    FailedScenarioPhase2,
    PayoutSuccess,
    PayoutDeclined,
    ProfitSplitDay,
    RevenueShareTaken,
}

impl EmailType {
    pub fn get_all() -> Vec<EmailType> {
        let enums = vec![
            Self::RegistrationConfirmation,
            Self::ForgotPassword,
            Self::PasswordResetNotification,
            Self::SuccessfulScenarioPhase1,
            Self::SuccessfulScenarioPhase2,
            Self::SuccessfulScenarioPhase2KycRequired,
            Self::SuccessfulScenarioPhase2ContractRequired,
            Self::SuccessfulProofOfIdentity,
            Self::RetryProofOfIdentity,
            Self::FailedProofOfIdentity,
            Self::SuccessfulProofOfAddress,
            Self::RetryProofOfAddress,
            Self::FailedProofOfAddress,
            Self::ContractToClient,
            Self::ServiceDeniedDueToViolationTermsAndCondOnContractSigning,
            Self::ServiceDeniedOnAllAccountsDueToViolationTermsAndCond,
            Self::ServiceDeniedDueToViolationOfOurTermsAndCond,
            Self::ServiceDeniedDueToViolationOfOurTermsAndCondFutureProfitBlocked,
            Self::ServiceDeniedDueToViolationOfOurTermsAndCondFutureAndCurrentProfitBlocked,
            Self::LiveAccountGranted,
            Self::LiveAccountLost ,
            Self::PaymentReceived,
            Self::PaymentFailed,
            Self::FailedScenarioPhase1,
            Self::FailedScenarioPhase2,
            Self::PayoutSuccess,
            Self::PayoutDeclined,
            Self::ProfitSplitDay,
            Self::RevenueShareTaken,
            ];
        return enums;
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::RegistrationConfirmation => "RegistrationConfirmation",
            Self::ForgotPassword => "ForgotPassword",
            Self::PasswordResetNotification => "PasswordResetNotification",
            Self::SuccessfulScenarioPhase1 => "SuccessfulScenarioPhase1",
            Self::SuccessfulScenarioPhase2 => "SuccessfulScenarioPhase2",
            Self::SuccessfulScenarioPhase2KycRequired => "SuccessfulScenarioPhase2KycRequired",
            Self::SuccessfulScenarioPhase2ContractRequired => "SuccessfulScenarioPhase2ContractRequired",
            Self::SuccessfulProofOfIdentity => "SuccessfulProofOfIdentity",
            Self::RetryProofOfIdentity => "RetryProofOfIdentity",
            Self::FailedProofOfIdentity => "FailedProofOfIdentity",
            Self::SuccessfulProofOfAddress => "SuccessfulProofOfAddress",
            Self::RetryProofOfAddress => "RetryProofOfAddress",
            Self::FailedProofOfAddress => "FailedProofOfAddress",
            Self::ContractToClient => "ContractToClient",
            Self::ServiceDeniedDueToViolationTermsAndCondOnContractSigning => "ServiceDeniedDueToViolationTermsAndCondOnContractSigning",
            Self::ServiceDeniedOnAllAccountsDueToViolationTermsAndCond => "ServiceDeniedOnAllAccountsDueToViolationTermsAndCond",
            Self::ServiceDeniedDueToViolationOfOurTermsAndCond => "ServiceDeniedDueToViolationOfOurTermsAndCond",
            Self::ServiceDeniedDueToViolationOfOurTermsAndCondFutureProfitBlocked => "ServiceDeniedDueToViolationOfOurTermsAndCondFutureProfitBlocked",
            Self::ServiceDeniedDueToViolationOfOurTermsAndCondFutureAndCurrentProfitBlocked => "ServiceDeniedDueToViolationOfOurTermsAndCondFutureAndCurrentProfitBlocked",
            Self::LiveAccountGranted => "LiveAccountGranted",
            Self::LiveAccountLost => "LiveAccountLost",
            Self::PaymentReceived => "PaymentReceived",
            Self::PaymentFailed => "PaymentFailed",
            Self::FailedScenarioPhase1 => "FailedScenarioPhase1",
            Self::FailedScenarioPhase2 => "FailedScenarioPhase2",
            Self::PayoutSuccess => "PayoutSuccess",
            Self::PayoutDeclined => "PayoutDeclined",
            Self::ProfitSplitDay => "ProfitSplitDay",
            Self::RevenueShareTaken => "RevenueShareTaken",

        }
    }
}

impl ToString for EmailType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}



