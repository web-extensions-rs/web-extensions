use super::{prelude::*, Status};

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-Tab>
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub active: bool,
    pub audible: Option<bool>,
    pub auto_discardable: bool,
    pub discarded: bool,
    pub fav_icon_url: Option<String>,
    pub group_id: i32,
    pub height: Option<u32>,
    pub highlighted: bool,
    pub id: Option<TabId>,
    pub incognito: bool,
    pub index: u32,
    // TODO: muted_info
    pub opener_tab_id: Option<TabId>,
    pub pending_url: Option<String>,
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
            audible: info.audible(),
            auto_discardable: info.auto_discardable(),
            discarded: info.discarded(),
            fav_icon_url: info.fav_icon_url(),
            group_id: info.group_id(),
            height: info.height(),
            highlighted: info.highlighted(),
            incognito: info.incognito(),
            index: info.index(),
            pending_url: info.pending_url(),
            pinned: info.pinned(),
            session_id: info.session_id(),
            title: info.title(),
            url: info.url(),
            width: info.width(),
            window_id: info.window_id(),
        }
    }
}
