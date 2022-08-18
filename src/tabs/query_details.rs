use super::{prelude::*, Status, WindowType};

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-query-queryInfo>
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDetails<'a> {
    pub active: Option<bool>,
    pub audible: Option<bool>,
    pub auto_discardable: Option<bool>,
    pub cookie_store_id: Option<&'a str>,
    pub current_window: Option<bool>,
    pub discarded: Option<bool>,
    pub hidden: Option<bool>,
    pub highlighted: Option<bool>,
    pub index: Option<i32>,
    pub muted: Option<bool>,
    pub last_focused_window: Option<bool>,
    pub pinned: Option<bool>,
    pub status: Option<Status>,
    pub title: Option<&'a str>,
    pub url: Option<&'a str>,
    pub window_id: i32,
    pub window_type: Option<WindowType>,
}
