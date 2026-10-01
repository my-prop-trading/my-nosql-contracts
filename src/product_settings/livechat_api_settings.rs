use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// LiveChat server-side API credentials (PROP25-1705). Editable from the back office
// (Integration Settings -> Chat tab) via backoffice-flows-grpc, and read live by the
// LiveChat integration services: livechat-webhook verifies the inbound webhook secret,
// livechat-bridge-flows-grpc uses api_url + basic_auth to call the LiveChat API (get_chat).
// Kept in its OWN "integrations" partition of the shared product-settings table so the
// credentials do not mix with the brand/chat-widget settings.
#[enum_model(partition_key: "integrations", row_key: "livechat")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LiveChatApiSettingsModel {
    /// LiveChat API base url, e.g. "https://api.livechatinc.com".
    pub api_url: Option<String>,
    /// base64("account_id:token") used as `Authorization: Basic <...>` for the LiveChat API.
    pub basic_auth: Option<String>,
    /// Shared secret LiveChat sends in every webhook body; verified before a payload is trusted.
    pub webhook_secret: Option<String>,
}
