use std::collections::HashMap;
use serde::*;

use crate::EmailTypeMyNoSql;

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum ReceiverType {
    Nobody = 0,
    All = 1,
    OnlyManagers = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateDetails {
    pub template_id: String,
    pub enabled: ReceiverType,
}

#[enum_model(partition_key:"email-provider", row_key: "send-grid-extended")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SendGridExtendingSettingsModel {
    pub send_grid_api_key: String,
    pub templates: HashMap<String, HashMap<String, TemplateDetails>>,
}

impl SendGridExtendingSettingsModel {
    pub fn get_email_id(&self, language: &str, email_type: EmailTypeMyNoSql) -> Option<String> {
        let templates = self.templates.get(language)?;
        let result = templates.get(email_type.as_str())?;
        Some(result.template_id.to_string())
    }
}
