use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onDetached>
pub fn on_detached() -> OnDetached {
    OnDetached(tabs().on_detached())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onDetached>
pub struct OnDetached(sys::EventTarget);

pub struct OnDetachedEventListener<'a>(EventListener<'a, dyn FnMut(i32, sys::TabDetachInfo)>);

impl OnDetachedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnDetached {
    pub fn add_listener<L>(&self, mut listener: L) -> OnDetachedEventListener
    where
        L: FnMut(i32, DetachInfo) + 'static,
    {
        let listener = Closure::new(move |tab_id: i32, info: sys::TabDetachInfo| {
            listener(tab_id, DetachInfo::from(info))
        });
        OnDetachedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetachInfo {
    pub old_window_id: i32,
    pub old_position: u32,
}

impl From<sys::TabDetachInfo> for DetachInfo {
    fn from(info: sys::TabDetachInfo) -> Self {
        Self {
            old_position: info.old_position(),
            old_window_id: info.old_window_id(),
        }
    }
}
