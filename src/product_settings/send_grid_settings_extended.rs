use std::collections::HashMap;


use serde::*;

use crate::EmailTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum ReceiverType {
    Nobody,
    All,
    OnlyManagers,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateDetails {
    template_id: String,
    enabled: ReceiverType,
}

#[enum_model(partition_key:"email-provider", row_key: "send-grid-extended")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SendGridSettingsExtendedModel {
    pub send_grid_api_key: String,
    pub templates: HashMap<String, HashMap<String, TemplateDetails>>,
}

impl SendGridSettingsModel {
    pub fn get_email_id(&self, language: &str, email_type: EmailTypeMyNoSql) -> Option<String> {
        let templates = self.templates.get(language)?;
        let result = templates.get(email_type.as_str())?;
        Some(result.to_string())
    }
}
