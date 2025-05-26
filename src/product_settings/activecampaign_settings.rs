use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"affiliate", row_key: "activecampaign")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveCampaignSettingsModel {
    pub api_key: String,
    pub url: String,
    pub event_actid: String,
    pub event_key: String,
    pub backoffice_url: String,
    pub default_account_owner: String,
    pub deal_stage_status_index: ActiveCampaignDealStageStatus,
    pub deal_group_index: ActiveCampaignDealGroup,
}

#[derive(Serialize, Deserialize Clone)]
pub struct ActiveCampaignDealStageStatus {
    // Payins
    pub successful: String,
    pub pending: String,
    pub unpaid: String,
    pub cancelled: String,
    pub declined: String,
    // Phase 1
    pub challenge_phase1_created: String,
    pub challenge_phase1_initiated: String,
    pub challenge_phase1_passed: String,
    pub challenge_phase1_failed: String,
    pub challenge_phase1_blocked: String,
    // Phase 2
    pub challenge_phase2_created: String,
    pub challenge_phase2_initiated: String,
    pub challenge_phase2_passed: String,
    pub challenge_phase2_failed: String,
    pub challenge_phase2_blocked: String,
    // Instant
    pub challenge_instant_created: String,
    pub challenge_instant_initiated: String,
    pub challenge_instant_failed: String,
    pub challenge_instant_blocked: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActiveCampaignDealGroup {
    pub payin: String,
    pub phase1_challenge: String,
    pub phase2_challenge: String,
    pub phase3_challenge: String,
    pub instant: String,
}