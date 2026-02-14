//! Wrapper for the [`chrome.omnibox` API](https://developer.chrome.com/docs/extensions/reference/omnibox/).

use derive_more::Display;
use gloo_console as console;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_extensions_sys as sys;

use crate::{event_listener::EventListener, util::*, Error};

/// Sets the description and styling for the default suggestion.
///
/// The default suggestion is the text that is displayed in the first suggestion row underneath the URL bar.
///
/// <https://developer.chrome.com/docs/extensions/reference/omnibox/#method-setDefaultSuggestion>
pub fn set_default_suggestion(suggestion: &DefaultSuggestResult<'_>) -> Result<(), Error> {
    let js_suggestion = js_from_serde(suggestion)?;

    sys::chrome()
        .omnibox()
        .set_default_suggestion(&js_suggestion);

    Ok(())
}

/// A suggest result.
///
/// <https://developer.chrome.com/docs/extensions/reference/history/#type-search-query>
#[derive(Debug, Clone, Serialize)]
pub struct DefaultSuggestResult<'a> {
    /// The text that is displayed in the URL dropdown.
    pub description: &'a str,
}

/// The style type.
///
/// <https://developer.chrome.com/docs/extensions/reference/history/#type-search-query>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DescriptionStyleType {
    Url,
    Match,
    Dim,
}

impl DescriptionStyleType {
    pub const DIM: &str = "dim";
    pub const MATCH: &str = "match";
    pub const URL: &str = "url";
}

impl TryFrom<JsValue> for DescriptionStyleType {
    type Error = Error;

    fn try_from(v: JsValue) -> Result<Self, Self::Error> {
        v.as_string()
            .and_then(|s| match s.as_str() {
                Self::DIM => Some(DescriptionStyleType::Dim),
                Self::MATCH => Some(DescriptionStyleType::Match),
                Self::URL => Some(DescriptionStyleType::Url),
                _ => None,
            })
            .ok_or(Error::Js(v))
    }
}
/// The window disposition for the omnibox query.
///
/// This is the recommended context to display results.
/// For example, if the omnibox command is to navigate to a certain URL,
/// a disposition of 'newForegroundTab' means the navigation should take place in a new selected tab.
///
/// <https://developer.chrome.com/docs/extensions/reference/omnibox/#type-OnInputEnteredDisposition>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OnInputEnteredDisposition {
    CurrentTab,
    NewForegroundTab,
    NewBackgroundTab,
}

impl OnInputEnteredDisposition {
    pub const CURRENT_TAB: &str = "currentTab";
    pub const NEW_BACKGROUND_TAB: &str = "newBackgroundTab";
    pub const NEW_FOREGROUND_TAB: &str = "newForegroundTab";
}

impl TryFrom<JsValue> for OnInputEnteredDisposition {
    type Error = Error;

    fn try_from(v: JsValue) -> Result<Self, Self::Error> {
        v.as_string()
            .and_then(|s| match s.as_str() {
                Self::CURRENT_TAB => Some(OnInputEnteredDisposition::CurrentTab),
                Self::NEW_BACKGROUND_TAB => Some(OnInputEnteredDisposition::NewBackgroundTab),
                Self::NEW_FOREGROUND_TAB => Some(OnInputEnteredDisposition::NewForegroundTab),
                _ => None,
            })
            .ok_or(Error::Js(v))
    }
}

/// A suggest result.
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestResult<'a> {
    /// The text that is put into the URL bar, and that is sent to the extension when the user chooses this entry.
    pub content: &'a str,
    /// Whether the suggest result can be deleted by the user.
    pub deletable: bool,
    /// The text that is displayed in the URL dropdown.
    pub description: &'a str,
}

/// User has deleted a suggested result.
pub fn on_delete_suggestion() -> OnDeleteSuggestion {
    OnDeleteSuggestion(sys::chrome().omnibox().on_delete_suggestion())
}

pub struct OnDeleteSuggestion(sys::EventTarget);

impl OnDeleteSuggestion {
    pub fn add_listener<L>(&self, mut listener: L) -> OnDeleteSuggestionListener
    where
        L: FnMut(&str) + 'static,
    {
        let listener = Closure::new(move |text: String| {
            console::debug!("on delete suggestion", sys::chrome().omnibox(), &text);

            listener(text.as_str())
        });

        OnDeleteSuggestionListener(EventListener::raw_new(&self.0, listener))
    }
}

pub struct OnDeleteSuggestionListener<'a>(EventListener<'a, dyn FnMut(String)>);

impl OnDeleteSuggestionListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

/// User has ended the keyword input session without accepting the input.
pub fn on_input_cancelled() -> OnInputCancelled {
    OnInputCancelled(sys::chrome().omnibox().on_input_cancelled())
}

pub struct OnInputCancelled(sys::EventTarget);

impl OnInputCancelled {
    pub fn add_listener<L>(&self, mut listener: L) -> OnInputCancelledListener
    where
        L: FnMut() + 'static,
    {
        let listener = Closure::new(move || {
            console::debug!("on input cancelled", sys::chrome().omnibox());

            listener()
        });

        OnInputCancelledListener(EventListener::raw_new(&self.0, listener))
    }
}

pub struct OnInputCancelledListener<'a>(EventListener<'a, dyn FnMut()>);

impl OnInputCancelledListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

/// User has changed what is typed into the omnibox.
pub fn on_input_changed() -> OnInputChanged {
    OnInputChanged(sys::chrome().omnibox().on_input_changed())
}

pub struct OnInputChanged(sys::EventTarget);

impl OnInputChanged {
    pub fn add_listener<L>(&self, mut listener: L) -> OnInputChangedListener
    where
        L: FnMut(&str, &mut (dyn FnMut(Vec<SuggestResult>) -> Result<(), Error> + 'static))
            + 'static,
    {
        let listener = Closure::new(move |text: String, suggest: Function| {
            console::debug!("on input changed", sys::chrome().omnibox(), &text, &suggest);

            let mut f = move |results: Vec<SuggestResult>| -> Result<(), Error> {
                let this = JsValue::null();
                let js_results = js_from_serde(&results).unwrap();

                suggest.call1(&this, &js_results)?;

                Ok(())
            };

            listener(text.as_str(), &mut f)
        });

        OnInputChangedListener(EventListener::raw_new(&self.0, listener))
    }
}

pub struct OnInputChangedListener<'a>(EventListener<'a, dyn FnMut(String, Function)>);

impl OnInputChangedListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

/// User has accepted what is typed into the omnibox.
pub fn on_input_entered() -> OnInputEntered {
    OnInputEntered(sys::chrome().omnibox().on_input_entered())
}

pub struct OnInputEntered(sys::EventTarget);

impl OnInputEntered {
    pub fn add_listener<L>(&self, mut listener: L) -> OnInputEnteredListener
    where
        L: FnMut(&str, OnInputEnteredDisposition) + 'static,
    {
        let callback = Closure::new(move |text: String, disposition: JsValue| {
            console::debug!(
                "on input entered",
                sys::chrome().omnibox(),
                &text,
                &disposition
            );

            listener(text.as_str(), disposition.try_into().unwrap())
        });

        OnInputEnteredListener(EventListener::raw_new(&self.0, callback))
    }
}

pub struct OnInputEnteredListener<'a>(EventListener<'a, dyn FnMut(String, JsValue)>);

impl OnInputEnteredListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}

/// User has ended the keyword input session without accepting the input.
pub fn on_input_started() -> OnInputStarted {
    OnInputStarted(sys::chrome().omnibox().on_input_started())
}

pub struct OnInputStarted(sys::EventTarget);

impl OnInputStarted {
    pub fn add_listener<L>(&self, mut listener: L) -> OnInputStartedListener
    where
        L: FnMut() + 'static,
    {
        let listener = Closure::new(move || {
            console::debug!("on input started", sys::chrome().omnibox());

            listener()
        });

        OnInputStartedListener(EventListener::raw_new(&self.0, listener))
    }
}

pub struct OnInputStartedListener<'a>(EventListener<'a, dyn FnMut()>);

impl OnInputStartedListener<'_> {
    pub fn forget(self) {
        self.0.forget()
    }
}
