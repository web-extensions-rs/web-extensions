use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onRemoved>
pub fn on_removed() -> OnRemoved {
    OnRemoved(tabs().on_removed())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onRemoved>
pub struct OnRemoved(sys::EventTarget);

pub struct OnRemovedEventListener<'a>(EventListener<'a, dyn FnMut(i32, sys::TabRemoveInfo)>);

impl OnRemovedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnRemoved {
    pub fn add_listener<L>(&self, mut listener: L) -> OnRemovedEventListener
    where
        L: FnMut(TabId, RemoveInfo) + 'static,
    {
        let listener = Closure::new(move |tab_id: i32, info: sys::TabRemoveInfo| {
            listener(TabId::from(tab_id), RemoveInfo::from(info))
        });
        OnRemovedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInfo {
    pub window_id: i32,
    pub is_window_closing: bool,
}

impl From<sys::TabRemoveInfo> for RemoveInfo {
    fn from(info: sys::TabRemoveInfo) -> Self {
        Self {
            window_id: info.window_id(),
            is_window_closing: info.is_window_closing(),
        }
    }
}
