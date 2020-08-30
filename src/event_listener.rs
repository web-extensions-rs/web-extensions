use std::marker::Unsize;
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Adapted from https://github.com/rustwasm/gloo/blob/2c9e776701ecb90c53e62dec1abd19c2b70e47c7/crates/events/src/lib.rs#L232-L582
#[must_use = "event listener will never be called after being dropped"]
pub struct EventListener<'a, F: ?Sized> {
    target: &'a web_extensions_sys::EventTarget,
    callback: Option<Closure<F>>,
}

impl<'a, F> EventListener<'a, F>
where
    F: ?Sized,
{
    #[inline]
    fn raw_new(target: &'a web_extensions_sys::EventTarget, callback: Closure<F>) -> Self {
        target.add_listener(callback.as_ref().unchecked_ref());
        Self {
            target,
            callback: Some(callback),
        }
    }
}

impl<'a, F> EventListener<'a, F>
where
    F: ?Sized + WasmClosure,
{
    #[inline]
    pub fn new<L>(target: &'a web_extensions_sys::EventTarget, callback: L) -> Self
    where
        L: Unsize<F> + 'static,
    {
        Self::raw_new(target, Closure::new(callback))
    }

    /// Keeps the `EventListener` alive forever, so it will never be dropped.
    ///
    /// This should only be used when you want the `EventListener` to last forever, otherwise it will leak memory!
    #[inline]
    pub fn forget(mut self) {
        // take() is necessary because of Rust's restrictions about Drop
        if let Some(callback) = self.callback.take() {
            callback.forget();
        }
    }
}

impl<F: ?Sized> Drop for EventListener<'_, F> {
    #[inline]
    fn drop(&mut self) {
        if let Some(callback) = &self.callback {
            self.target
                .remove_listener(callback.as_ref().unchecked_ref());
        }
    }
}
