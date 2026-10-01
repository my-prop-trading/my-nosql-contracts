use serde::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatWidget {
    pub live_chat: Option<ChatSetting>,
    pub zen_desk: Option<ChatSetting>,
    pub hide_widget: Option<bool>,
    pub chat_type: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatSetting {
    /// Public widget key/license embedded in the front-end chat widget.
    pub key: String,

    // --- LiveChat server-side API credentials (PROP25-1705) ---
    // Edited from the back office (Integration Settings -> Chat tab) and read by
    // livechat-bridge-flows-grpc to call the LiveChat API and verify inbound webhooks.
    // All optional: zen_desk and legacy records that only carry `Key` keep deserializing.
    /// LiveChat API base url, e.g. "https://api.livechatinc.com".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    /// base64("account_id:token") used as `Authorization: Basic <...>` for the LiveChat API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<String>,
    /// Shared secret LiveChat sends in every webhook body; verified before trusting a payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}