use serde::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatWidget {
    pub live_chat: Option<ChatSetting>,
    pub zen_desk: Option<ChatSetting>,
    pub hide_widget: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatSetting {
    pub key: String,
}