use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onActivated>
pub fn on_activated() -> OnActivated {
    OnActivated(tabs().on_activated())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onActivated>
pub struct OnActivated(sys::EventTarget);

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-onActivated-callback>
pub struct OnActivatedEventListener<'a>(EventListener<'a, dyn FnMut(sys::TabActiveInfo)>);

impl OnActivatedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnActivated {
    pub fn add_listener<L>(&self, mut listener: L) -> OnActivatedEventListener
    where
        L: FnMut(ActiveInfo) + 'static,
    {
        let listener =
            Closure::new(move |info: sys::TabActiveInfo| listener(ActiveInfo::from(info)));
        OnActivatedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-onActivated-callback-activeInfo>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveInfo {
    pub tab_id: TabId,
    pub window_id: i32,
}

impl From<sys::TabActiveInfo> for ActiveInfo {
    fn from(info: sys::TabActiveInfo) -> Self {
        let tab_id = TabId::from(info.tab_id());
        ActiveInfo {
            tab_id,
            window_id: info.window_id(),
        }
    }
}
