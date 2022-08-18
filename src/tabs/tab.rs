use super::{prelude::*, Status};

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-Tab>
#[derive(Debug, PartialEq, Deserialize)]
pub struct Tab {
    pub active: bool,
    pub attention: Option<bool>,
    pub audible: Option<bool>,
    pub auto_discardable: Option<bool>,
    pub cookie_store_id: Option<String>,
    pub discarded: Option<bool>,
    pub fav_icon_url: Option<String>,
    pub height: Option<u32>,
    pub hidden: bool,
    pub highlighted: bool,
    pub id: Option<TabId>,
    pub incognito: bool,
    pub index: u32,
    pub is_article: bool,
    pub is_in_reader_mode: bool,
    pub last_accessed: f64,
    // TODO: muted_info
    pub opener_tab_id: Option<TabId>,
    pub pinned: bool,
    pub session_id: Option<String>,
    pub status: Option<Status>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub width: Option<u32>,
    pub window_id: i32,
}

impl From<sys::Tab> for Tab {
    fn from(info: sys::Tab) -> Self {
        let status = info.status().map(|s| Status::try_from(s).expect("status"));
        let id = info.id().map(TabId::from);
        let opener_tab_id = info.opener_tab_id().map(TabId::from);
        Self {
            id,
            opener_tab_id,
            status,
            active: info.active(),
            attention: info.attention(),
            audible: info.audible(),
            auto_discardable: info.auto_discardable(),
            cookie_store_id: info.cookie_store_id(),
            discarded: info.discarded(),
            fav_icon_url: info.fav_icon_url(),
            height: info.height(),
            hidden: info.hidden(),
            highlighted: info.highlighted(),
            incognito: info.incognito(),
            index: info.index(),
            is_article: info.is_article(),
            is_in_reader_mode: info.is_in_reader_mode(),
            last_accessed: info.last_accessed(),
            pinned: info.pinned(),
            session_id: info.session_id(),
            title: info.title(),
            url: info.url(),
            width: info.width(),
            window_id: info.window_id(),
        }
    }
}
