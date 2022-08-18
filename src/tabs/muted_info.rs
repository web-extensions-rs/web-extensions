use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-MutedInfo>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutedInfo {
    pub muted: bool,
    pub extension_id: Option<String>,
    pub reason: Option<String>,
}

impl From<sys::TabMutedInfo> for MutedInfo {
    fn from(info: sys::TabMutedInfo) -> Self {
        Self {
            muted: info.muted(),
            extension_id: info.extension_id(),
            reason: info.reason(),
        }
    }
}
