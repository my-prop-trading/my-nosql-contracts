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
    pub languages: HashMap<String, String>,
    pub styles_css_url: String,
    pub live_chat_license: Option<String>,
    pub hide_live_chat: Option<bool>,
    pub affiliate_url: Option<String>,
    pub hide_affiliate_menu: Option<bool>,
    pub discord_url: Option<String>,
    pub hide_discord_menu: Option<bool>,
    pub google_tag_manager_key: Option<String>,
    pub hide_google_tag: Option<bool>,
    pub show_live_chat: bool,
    pub hide_economic_calendar_menu: Option<bool>,
    pub disclaimer_url: Option<String>,
    pub cookie_policy_url: Option<String>,
    // email 
    pub email_from: String,
    pub email_from_name: Option<String>,
    pub contract_manager_email: Option<String>,
    pub email_cc: Option<String>,
    pub email_bcc: Option<String>,
    pub mail_logo_url: Option<String>,
    pub mail_success_picture_url: Option<String>,
    pub mail_fail_picture_url: Option<String>,
    // 
    pub registration_type: Option<String>,
}
