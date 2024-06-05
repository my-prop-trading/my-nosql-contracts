use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("features")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FeaturesMyNoSqlEntity {
    pub is_enabled: bool,
}

impl FeaturesMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(app_feature: AppFeature) -> StrOrString<'s> {
        let str: String = app_feature.into();
        str.into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AppFeature {
    DirectRegistration = 0,
    RegistrationThroughCheckout = 1,
}

impl Into<String> for AppFeature {
    fn into(self) -> String {
        match self {
            AppFeature::DirectRegistration => "direct-registration".into(),
            AppFeature::RegistrationThroughCheckout => "registration-through-checkout".into(),
        }
    }
}

impl AppFeature {
    pub fn from_str(app_feature: &str) -> Self {
        match app_feature {
            "direct-registration" => AppFeature::DirectRegistration,
            "registration-through-checkout" => AppFeature::RegistrationThroughCheckout,
            _ => AppFeature::DirectRegistration,
        }
    }
}
