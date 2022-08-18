use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onZoomChange>
pub fn on_zoom_change() -> OnZoomChange {
    OnZoomChange(tabs().on_zoom_change())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onZoomChange>
pub struct OnZoomChange(sys::EventTarget);

pub struct OnZoomChangeEventListener<'a>(EventListener<'a, dyn FnMut(sys::TabZoomChangeInfo)>);

impl OnZoomChangeEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnZoomChange {
    pub fn add_listener<L>(&self, mut listener: L) -> OnZoomChangeEventListener
    where
        L: FnMut(ZoomChangeInfo) + 'static,
    {
        let listener =
            Closure::new(move |info: sys::TabZoomChangeInfo| listener(ZoomChangeInfo::from(info)));
        OnZoomChangeEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoomChangeInfo {
    // TODO: Add more fields from https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/onZoomChange#ZoomChangeInfo
    pub new_zoom_factor: f64,
    pub old_zoom_factor: f64,
    pub tab_id: i32,
}

impl From<sys::TabZoomChangeInfo> for ZoomChangeInfo {
    fn from(info: sys::TabZoomChangeInfo) -> Self {
        Self {
            new_zoom_factor: info.new_zoom_factor(),
            old_zoom_factor: info.old_zoom_factor(),
            tab_id: info.tab_id(),
        }
    }
}
