#[derive(Debug)]
pub enum EmailTypeMyNoSql {
    RegistrationConfirmation = 0,
    ForgotPassword = 1,
    PasswordResetNotification = 2,
    SuccessfulScenarioPhase1 = 3,
    SuccessfulScenarioPhase2 = 4,
    SuccessfulScenarioPhase2KycRequired = 5,
    SuccessfulScenarioPhase2ContractRequired = 6,
    SuccessfulProofOfIdentity = 7,
    RetryProofOfIdentity = 8,
    FailedProofOfIdentity = 9,
    SuccessfulProofOfAddress = 10,
    RetryProofOfAddress = 11,
    FailedProofOfAddress = 12,
    ContractToClient = 13,
    ServiceDeniedDueToViolationTermsAndCondOnContractSigning = 14,
    ServiceDeniedOnAllAccountsDueToViolationTermsAndCond = 15,
    ServiceDeniedDueToViolationOfOurTermsAndCond = 16,
    ServiceDeniedDueToViolationOfOurTermsAndCondFutureProfitBlocked = 17,
    ServiceDeniedDueToViolationOfOurTermsAndCondFutureAndCurrentProfitBlocked = 18,
    LiveAccountGranted = 19,
    LiveAccountLost = 20,
    PaymentReceived = 21,
    PaymentFailed = 22,
    FailedScenarioPhase1 = 23,
    FailedScenarioPhase2 = 24,
    PayoutSuccess = 25,
    PayoutDeclined = 26,
    ProfitSplitDay = 27,
    RevenueShareTaken = 28,
}

impl EmailTypeMyNoSql {
    pub fn get_all() -> Vec<EmailTypeMyNoSql> {
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

impl ToString for EmailTypeMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for EmailTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => EmailTypeMyNoSql::RegistrationConfirmation,
            1 => EmailTypeMyNoSql::ForgotPassword,
            2 => EmailTypeMyNoSql::PasswordResetNotification,
            3 => EmailTypeMyNoSql::SuccessfulScenarioPhase1,
            4 => EmailTypeMyNoSql::SuccessfulScenarioPhase2,
            5 => EmailTypeMyNoSql::SuccessfulScenarioPhase2KycRequired,
            6 => EmailTypeMyNoSql::SuccessfulScenarioPhase2ContractRequired,
            7 => EmailTypeMyNoSql::SuccessfulProofOfIdentity,
            8 => EmailTypeMyNoSql::RetryProofOfIdentity,
            9 => EmailTypeMyNoSql::FailedProofOfIdentity,
            10 => EmailTypeMyNoSql::SuccessfulProofOfAddress,
            11 => EmailTypeMyNoSql::RetryProofOfAddress,
            12 => EmailTypeMyNoSql::FailedProofOfAddress,
            13 => EmailTypeMyNoSql::ContractToClient,
            14 => EmailTypeMyNoSql::ServiceDeniedDueToViolationTermsAndCondOnContractSigning,
            15 => EmailTypeMyNoSql::ServiceDeniedOnAllAccountsDueToViolationTermsAndCond,
            16 => EmailTypeMyNoSql::ServiceDeniedDueToViolationOfOurTermsAndCond,
            17 => EmailTypeMyNoSql::ServiceDeniedDueToViolationOfOurTermsAndCondFutureProfitBlocked,
            18 => EmailTypeMyNoSql::ServiceDeniedDueToViolationOfOurTermsAndCondFutureAndCurrentProfitBlocked,
            19 => EmailTypeMyNoSql::LiveAccountGranted,
            20 => EmailTypeMyNoSql::LiveAccountLost,
            21 => EmailTypeMyNoSql::PaymentReceived,
            22 => EmailTypeMyNoSql::PaymentFailed,
            23 => EmailTypeMyNoSql::FailedScenarioPhase1,
            24 => EmailTypeMyNoSql::FailedScenarioPhase2,
            25 => EmailTypeMyNoSql::PayoutSuccess,
            26 => EmailTypeMyNoSql::PayoutDeclined,
            27 => EmailTypeMyNoSql::ProfitSplitDay,
            28 => EmailTypeMyNoSql::RevenueShareTaken,
            _ => panic!(
                "Invalid value '{}' for EmailTypeMyNoSql",
                value,
            )
        }
    }
}



