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
        sys::chrome.tabs()
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

#[cfg(test)]
mod tests;

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
