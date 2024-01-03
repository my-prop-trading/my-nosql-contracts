use std::collections::HashMap;


use serde::*;

use crate::EmailTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"email-provider", row_key: "send-grid")]
#[derive(Serialize, Deserialize, Clone)]
pub struct SendGridSettingsModel {
    pub send_grid_api_key: String,
    pub email_from: String,
    pub email_bcc: Option<String>,
    pub templates: HashMap<String, String>,
}

impl SendGridSettingsModel {
    pub fn get_email_id(&self, email_type: EmailTypeMyNoSql) -> Option<String> {
        let result = self.templates.get(email_type.as_str())?;
        Some(result.to_string())
    }
}
