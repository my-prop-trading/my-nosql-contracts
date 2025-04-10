use serde::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatWidget {
    pub live_chat: Option<ChatSetting>,
    pub zen_desk: Option<ChatSetting>,
    pub hide_widget: Option<bool>,
    pub chat_type: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatSetting {
    pub key: String,
}