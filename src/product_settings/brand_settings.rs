use serde::*;
use std::collections::HashMap;

service_sdk::macros::use_my_no_sql_entity!();
#[enum_model(partition_key:"settings", row_key: "brand")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BrandSettingsModel {
    pub logo_url: String,
    pub policy_url: String,
    pub terms_url: String,
    pub about_url: String,
    pub support_url: String,
    pub color_url: String,
    pub brand_name: String,
    pub brand_copyrights: String,
    pub favicon_url: String,
    pub refund_policy_url: String,
    pub title_url: String,
    pub trading_packages_url: String,
    pub home_page_url: String,
    pub verification_url: String,
    pub payouts_page_url: String,
    pub login_url: String,
    pub company_name: String,
    pub email_from: String,
    pub email_cc: Option<String>,
    pub email_bcc: Option<String>,
    pub languages: HashMap<String, String>,
    pub show_live_chat: bool,
    pub live_chat_license: Option<String>,
    pub styles_css: String
}
