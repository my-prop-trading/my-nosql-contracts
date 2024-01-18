use crate::SendGridSettingsModel;
use crate::BrandSettingsModel;
use crate::IdentomatSettingsModel;
use crate::KycSettingsModel;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_of_my_no_sql_entity(table_name:"product-settings", generate_unwraps)]
pub enum ProductSettings {
    SendGrid(SendGridSettingsModel),
    BrandSettings(BrandSettingsModel),
    IdentomatSettings(IdentomatSettingsModel),
    KycSettings(KycSettingsModel)
}
