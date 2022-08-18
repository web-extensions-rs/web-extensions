use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onAttached>
pub fn on_attached() -> OnAttached {
    OnAttached(tabs().on_attached())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onAttached>
pub struct OnAttached(sys::EventTarget);

pub struct OnAttachedEventListener<'a>(EventListener<'a, dyn FnMut(i32, sys::TabAttachInfo)>);

impl OnAttachedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnAttached {
    pub fn add_listener<L>(&self, mut listener: L) -> OnAttachedEventListener
    where
        L: FnMut(TabId, AttachInfo) + 'static,
    {
        let listener = Closure::new(move |tab_id: i32, info: sys::TabAttachInfo| {
            listener(TabId::from(tab_id), AttachInfo::from(info))
        });
        OnAttachedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachInfo {
    pub new_window_id: i32,
    pub new_position: u32,
}

impl From<sys::TabAttachInfo> for AttachInfo {
    fn from(info: sys::TabAttachInfo) -> Self {
        Self {
            new_position: info.new_position(),
            new_window_id: info.new_window_id(),
        }
    }
}
