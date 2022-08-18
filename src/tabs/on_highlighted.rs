use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onHighlighted>
pub fn on_highlighted() -> OnHighlighted {
    OnHighlighted(tabs().on_highlighted())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onHighlighted>
pub struct OnHighlighted(sys::EventTarget);

pub struct OnHighlightedEventListener<'a>(EventListener<'a, dyn FnMut(sys::TabHighlightInfo)>);

impl OnHighlightedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnHighlighted {
    pub fn add_listener<L>(&self, mut listener: L) -> OnHighlightedEventListener
    where
        L: FnMut(HighlightInfo) + 'static,
    {
        let listener =
            Closure::new(move |info: sys::TabHighlightInfo| listener(HighlightInfo::from(info)));
        OnHighlightedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightInfo {
    pub window_id: i32,
    pub tab_ids: Vec<TabId>,
}

impl From<sys::TabHighlightInfo> for HighlightInfo {
    fn from(info: sys::TabHighlightInfo) -> Self {
        let tab_ids = info.tab_ids().into_serde().expect("Tab IDs");
        Self {
            tab_ids,
            window_id: info.window_id(),
        }
    }
}
