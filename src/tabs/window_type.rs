use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-WindowType>
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum WindowType {
    #[serde(rename(serialize = "normal"))]
    Normal,
    #[serde(rename(serialize = "popup"))]
    Popup,
    #[serde(rename(serialize = "panel"))]
    Panel,
    #[serde(rename(serialize = "devtools"))]
    Devtools,
}
