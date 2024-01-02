use std::collections::HashMap;

use serde::*;

use crate::EmailType;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"email-provider", row_key: "send-grid")]
#[derive(Serialize, Deserialize, Clone)]
pub struct SendGridSettingsModel {
    pub send_grid_api_key: String,
    pub from_email: String,
    pub bcc_email: Option<String>,
    pub templates: HashMap<String, String>,
}

impl SendGridSettingsModel {
    pub fn get_email_id(&self, email_type: EmailType) -> Option<String> {
        let result = self.templates.get(email_type.as_str())?;
        Some(result.to_string())
    }
}
