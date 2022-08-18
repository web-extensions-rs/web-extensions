use super::{prelude::*, MutedInfo, Status, Tab};

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onUpdated>
pub fn on_updated() -> OnUpdated {
    OnUpdated(tabs().on_updated())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onUpdated>
pub struct OnUpdated(sys::EventTarget);

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-onUpdated-callback>
pub struct OnUpdatedEventListener<'a>(
    EventListener<'a, dyn FnMut(i32, sys::TabChangeInfo, sys::Tab)>,
);

impl OnUpdatedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnUpdated {
    pub fn add_listener<L>(&self, mut listener: L) -> OnUpdatedEventListener
    where
        L: FnMut(TabId, ChangeInfo, Tab) + 'static,
    {
        let listener = Closure::new(
            move |tab_id: i32, info: sys::TabChangeInfo, tab: sys::Tab| {
                listener(TabId::from(tab_id), ChangeInfo::from(info), Tab::from(tab))
            },
        );
        OnUpdatedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-onUpdated-callback-changeInfo>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInfo {
    /// The tab's new audible state.
    pub audible: Option<bool>,

    /// The tab's new auto-discardable state.
    pub auto_discardable: Option<bool>,

    /// The tab's new discarded state.
    pub discarded: Option<bool>,

    /// The tab's new favicon URL.
    pub fav_icon_url: Option<String>,

    /// The tab's new group.
    pub group_id: Option<i32>,

    /// The tab's new muted state and the reason for the change.
    pub muted_info: Option<MutedInfo>,

    /// The tab's new pinned state.
    pub pinned: Option<bool>,

    /// The tab's loading status.
    pub status: Option<Status>,

    /// The tab's new title.
    pub title: Option<String>,

    /// The tab's URL if it has changed.
    pub url: Option<String>,
}

impl From<sys::TabChangeInfo> for ChangeInfo {
    fn from(info: sys::TabChangeInfo) -> Self {
        let status = info.status().map(|s| Status::try_from(s).expect("status"));
        let muted_info = info.muted_info().map(MutedInfo::from);
        Self {
            status,
            muted_info,
            audible: info.audible(),
            auto_discardable: info.auto_discardable(),
            discarded: info.discarded(),
            fav_icon_url: info.fav_icon_url(),
            group_id: info.group_id(),
            pinned: info.pinned(),
            title: info.title(),
            url: info.url(),
        }
    }
}
