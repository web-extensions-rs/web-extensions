use super::prelude::*;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onReplaced>
pub fn on_replaced() -> OnReplaced {
    OnReplaced(tabs().on_replaced())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onReplaced>
pub struct OnReplaced(sys::EventTarget);

pub struct OnReplacedEventListener<'a>(EventListener<'a, dyn FnMut(i32, i32)>);

impl OnReplacedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnReplaced {
    pub fn add_listener<L>(&self, mut listener: L) -> OnReplacedEventListener
    where
        L: FnMut(ReplaceInfo) + 'static,
    {
        let listener = Closure::new(move |added_tab_id: i32, removed_tab_id: i32| {
            let replace_info = ReplaceInfo {
                added: TabId::from(added_tab_id),
                removed: TabId::from(removed_tab_id),
            };
            listener(replace_info)
        });
        OnReplacedEventListener(EventListener::raw_new(&self.0, listener))
    }
}

#[derive(Debug)]
pub struct ReplaceInfo {
    pub added: TabId,
    pub removed: TabId,
}
