//! Wrapper for the [`chrome.history` API](https://developer.chrome.com/docs/extensions/reference/history/).

use crate::{util::*, Error};
use serde::{Deserialize, Serialize};
use web_extensions_sys as sys;

/// <https://developer.chrome.com/docs/extensions/reference/history/#method-search>
pub async fn search(query: &Query<'_>) -> Result<Vec<HistoryItem>, Error> {
    let js_query = js_from_serde(query)?;
    let js_value = sys::chrome
        .history()
        .search(object_from_js(&js_query)?)
        .await;
    serde_from_js(js_value)
}

/// <https://developer.chrome.com/docs/extensions/reference/history/#type-search-query>
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query<'a> {
    /// A free-text query to the history service.
    ///
    /// Leave empty to retrieve all pages.
    pub text: &'a str,

    /// Limit results to those visited before this date,
    /// represented in milliseconds since the epoch.
    pub end_time: Option<i64>,

    /// The maximum number of results to retrieve.
    ///
    /// Defaults to 100.
    pub max_results: Option<usize>,

    /// Limit results to those visited after this date,
    /// represented in milliseconds since the epoch.
    ///
    /// If not specified, this defaults to 24 hours in the past.
    pub start_time: Option<i64>,
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(q: &'a str) -> Self {
        Self {
            text: q,
            ..Default::default()
        }
    }
}

/// <https://developer.chrome.com/docs/extensions/reference/history/#type-HistoryItem>
///
/// An object encapsulating one result of a history query.

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    /// Unique identifier.
    pub id: String,

    /// When this page was last loaded, represented in milliseconds since the epoch.
    pub last_visit_time: Option<i64>,

    /// The title of the page when it was last loaded.
    pub title: Option<String>,

    /// The number of times the user has navigated to this page by typing in the address.
    pub typed_count: Option<usize>,

    /// The URL of the page.
    pub url: Option<String>,

    /// The number of times the user has visited the page.
    pub visit_count: Option<usize>,
}
