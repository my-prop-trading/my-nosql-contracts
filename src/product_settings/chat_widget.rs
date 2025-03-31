#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChatWidget {
    pub live_chat: Option<LiveChat>,
    pub zend_desk: Option<ZendDesk>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LiveChat {
    pub license_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ZendDesk {
    pub key: String,
}