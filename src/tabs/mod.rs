//! Wrapper for the [`chrome.tabs` API](https://developer.chrome.com/docs/extensions/reference/tabs/).

pub(crate) mod prelude {
    pub(crate) use crate::util::{js_from_serde, object_from_js, serde_from_js_result};
    pub use crate::{event_listener::EventListener, tabs::TabId, Error};
    pub use serde::{Deserialize, Serialize};
    pub use wasm_bindgen::closure::Closure;
    pub use web_extensions_sys as sys;

    pub fn tabs() -> sys::Tabs {
        // Currently we assume a chrome browser and Manifest V3.
        //
        // Once MV3 is supported by FireFox, we need to check if we can use the same namespace,
        // a shim or our own implementation.
        sys::chrome().tabs()
    }
}

use self::prelude::*;

/// The ID of the tab.
///
/// Tab IDs are unique within a browser session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabId(i32);

impl From<i32> for TabId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<TabId> for i32 {
    fn from(id: TabId) -> Self {
        id.0
    }
}

mod on_activated;
mod on_attached;
mod on_created;
mod on_detached;
mod on_highlighted;
mod on_moved;
mod on_removed;
mod on_replaced;
mod on_updated;
mod on_zoom_change;

mod muted_info;
mod query_details;
mod status;
mod tab;
mod window_type;

pub use self::{
    muted_info::*, on_activated::*, on_attached::*, on_created::*, on_detached::*,
    on_highlighted::*, on_moved::*, on_removed::*, on_replaced::*, on_updated::*,
    on_zoom_change::*, query_details::*, status::*, tab::*, window_type::*,
};

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-get>
pub async fn get(tab_id: TabId) -> Result<Tab, Error> {
    let result = tabs().get(tab_id.0).await;
    serde_from_js_result(result)
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-query>
pub async fn query(details: &QueryDetails<'_>) -> Result<Vec<Tab>, Error> {
    let js_details = js_from_serde(details)?;
    let result = tabs().query(object_from_js(&js_details)?).await;
    serde_from_js_result(result)
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-sendMessage>
pub async fn send_message<T>(tab_id: TabId, message: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let js_message = js_from_serde(message)?;
    let options = None;
    tabs()
        .send_message(tab_id.0, &js_message, options)
        .await
        .map(|_| ())?;
    Ok(())
}

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-create>
pub async fn create(props: CreateProperties<'_>) -> Result<Tab, Error> {
    let js_props = js_from_serde(&props)?;
    let result = tabs().create(object_from_js(&js_props)?).await;
    serde_from_js_result(result)
}

/// Information necessary to open a new tab.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProperties<'a> {
    pub active: bool,
    pub url: &'a str,
}

/// Modifies the properties of a tab.
///
/// Properties that are not specified in updateProperties are not modified.
///
/// <https://developer.chrome.com/docs/extensions/reference/tabs/#method-update>
pub async fn update(tab_id: Option<TabId>, props: UpdateProperties<'_>) -> Result<Tab, Error> {
    let js_props = js_from_serde(&props)?;
    let result = tabs()
        .update(tab_id.map(|id| id.0), object_from_js(&js_props)?)
        .await;
    serde_from_js_result(result)
}

/// Information necessary to open a new tab.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProperties<'a> {
    /// Whether the tab should be active.
    pub active: Option<bool>,
    /// Whether the tab should be discarded automatically by the browser when resources are low.
    pub auto_discardable: Option<bool>,
    /// Adds or removes the tab from the current selection.
    pub highlighted: Option<bool>,
    /// Whether the tab should be muted.
    pub muted: Option<bool>,
    /// The ID of the tab that opened this tab.
    pub opener_tab_id: Option<TabId>,
    /// Whether the tab should be pinned.
    pub pinned: Option<bool>,
    /// Whether the tab should be selected.
    pub selected: Option<bool>,
    /// A URL to navigate the tab to.
    pub url: Option<&'a str>,
}
