use serde::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatWidget {
    pub live_chat: Option<LiveChat>,
    pub zen_desk: Option<ZenDesk>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LiveChat {
    pub license_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ZenDesk {
    pub key: String,
}