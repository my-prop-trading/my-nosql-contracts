use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"settings", row_key: "recaptcha")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RecaptchaSettingsModel {
    pub recaptcha_secret_key: String,
    pub recaptcha_site_key: String,
    pub recaptcha_threshold_score: f64,
}