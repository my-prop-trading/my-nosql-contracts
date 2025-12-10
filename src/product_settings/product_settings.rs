use crate::{CrossMarginSettingsModel, CtraderSettingsModel, SendGridSettingsModel, TradeLockerSettingsModel};
use crate::BrandSettingsModel;
use crate::IdentomatSettingsModel;
use crate::KycSettingsModel;
use crate::TradingPackagesSettingsModel;
use crate::AcqufySettingsModel;
use crate::RecaptchaSettingsModel;
use crate::GoogleAnalyticsSettingsModel;
use crate::TradingPlatformSettingsModel;
use crate::KeitaroSettingsModel;
use crate::GoogleAnalyticsAffiliateSettingsModel;
use crate::AffiliateSettingsModel;
use crate::MetaPixelSettingsModel;
use crate::product_settings::bridgerpay_settings::BridgerpaySettingsModel;
use crate::SendGridExtendingSettingsModel;
use crate::ActiveCampaignSettingsModel;
use crate::SentrySettingsModel;
use crate::PixelTikTokSettingsModel;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_of_my_no_sql_entity(table_name:"product-settings", generate_unwraps)]
pub enum ProductSettings {
    SendGrid(SendGridSettingsModel),
    BrandSettings(BrandSettingsModel),
    IdentomatSettings(IdentomatSettingsModel),
    KycSettings(KycSettingsModel),
    TradingPackagesSettingsModel(TradingPackagesSettingsModel),
    Acqufy(AcqufySettingsModel),
    CtraderSettings(CtraderSettingsModel),
    CrossMarginSettings(CrossMarginSettingsModel),
    RecaptchaSettings(RecaptchaSettingsModel),
    GoogleAnalyticsSettings(GoogleAnalyticsSettingsModel),
    SentrySettings(SentrySettingsModel),
    TradingPlatformSettings(TradingPlatformSettingsModel),
    TradeLockerSettings(TradeLockerSettingsModel),
    KeitaroSettingsModel(KeitaroSettingsModel),
    GoogleAnalyticsAffiliateSettingsModel(GoogleAnalyticsAffiliateSettingsModel),
    AffiliateSettings(AffiliateSettingsModel),
    MetaPixelSettings(MetaPixelSettingsModel),
    BridgerpaySettings(BridgerpaySettingsModel),
    SendGridExtendingSettings(SendGridExtendingSettingsModel),
    ActiveCampaignSettings(ActiveCampaignSettingsModel),
    PixelTikTokSettings(PixelTikTokSettingsModel),
}
