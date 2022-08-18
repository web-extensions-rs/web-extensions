use super::{prelude::*, Tab};

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onCreated>
pub fn on_created() -> OnCreated {
    OnCreated(tabs().on_created())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#event-onCreated>
pub struct OnCreated(sys::EventTarget);

pub struct OnCreatedEventListener<'a>(EventListener<'a, dyn FnMut(sys::Tab)>);

impl OnCreatedEventListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

impl OnCreated {
    pub fn add_listener<L>(&self, mut listener: L) -> OnCreatedEventListener
    where
        L: FnMut(Tab) + 'static,
    {
        let listener = Closure::new(move |tab: sys::Tab| listener(Tab::from(tab)));
        OnCreatedEventListener(EventListener::raw_new(&self.0, listener))
    }
}
