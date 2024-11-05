use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(i32)]
pub enum AffiliateProviderTypeMyNoSql {
    Acqufy = 0,
    Keitaro = 1,
}

#[enum_model(partition_key:"settings", row_key: "affiliate")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AffiliateSettingsModel {
    pub provider: AffiliateProviderTypeMyNoSql,
}

impl AffiliateSettingsModel {
    pub fn as_str(&self) -> &str {
        match self.provider {
            AffiliateProviderTypeMyNoSql::Acqufy => "acqufy",
            AffiliateProviderTypeMyNoSql::Keitaro => "keitaro",    
        }
    }
}