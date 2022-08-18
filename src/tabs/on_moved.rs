use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onMoved>
pub fn on_moved() -> OnMoved {
    OnMoved(tabs().on_moved())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onMoved>
pub struct OnMoved(sys::EventTarget);

pub struct OnMovedEventListener<'a>(EventListener<'a, dyn FnMut(i32, sys::TabMoveInfo)>);

impl OnMovedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnMoved {
    pub fn add_listener<L>(&self, mut listener: L) -> OnMovedEventListener
    where
        L: FnMut(TabId, MoveInfo) + 'static,
    {
        let listener = Closure::new(move |tab_id: i32, tab: sys::TabMoveInfo| {
            listener(TabId::from(tab_id), MoveInfo::from(tab))
        });
        OnMovedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveInfo {
    pub window_id: i32,
    pub from_index: u32,
    pub to_index: u32,
}

impl From<sys::TabMoveInfo> for MoveInfo {
    fn from(info: sys::TabMoveInfo) -> Self {
        Self {
            from_index: info.from_index(),
            to_index: info.to_index(),
            window_id: info.window_id(),
        }
    }
}
