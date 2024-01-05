use std::collections::HashMap;


use serde::*;

use crate::EmailTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"email-provider", row_key: "send-grid")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SendGridSettingsModel {
    pub send_grid_api_key: String,
    pub email_from: String,
    pub email_bcc: Option<String>,
    pub templates: HashMap<String, HashMap<String, String>>,
    pub trading_packages_link: String,
    pub company_name: String,
    pub support_link: String,
    pub home_page_link: String,
    pub verification_link: String,
    pub payouts_page_link: String,
}

impl SendGridSettingsModel {
    pub fn get_email_id(&self, language: &str, email_type: EmailTypeMyNoSql) -> Option<String> {
        let templates = self.templates.get(language)?;
        let result = templates.items.get(email_type.as_str())?;
        Some(result.to_string())
    }
}
