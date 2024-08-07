use serde::*;
use std::collections::HashMap;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("brands")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BrandSettingsNoSqlEntity {
    pub brand_id: String,
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
    pub styles_css_url: String,
    pub live_chat_license: Option<String>,
    pub hide_live_chat: Option<bool>,
    pub hide_affiliate_menu: Option<bool>,
    pub mail_logo_url: Option<String>,
    pub mail_success_picture_url: Option<String>,
    pub mail_fail_picture_url: Option<String>,
    pub discord_url: Option<String>,
    pub hide_discord: Option<bool>,
    pub google_tag_manager_key: Option<String>,
    pub hide_google_tag: Option<String>,
}

impl BrandSettingsNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "brand";

    pub fn get_domain(&self) -> &str {
        self.row_key.as_str()
    }
}