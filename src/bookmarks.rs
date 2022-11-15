//! Wrapper for the [`chrome.bookmarks` API](https://developer.chrome.com/docs/extensions/reference/bookmarks/).

use crate::{util::*, Error};
use serde::{Deserialize, Serialize};
use web_extensions_sys as sys;

/// <https://developer.chrome.com/docs/extensions/reference/bookmarks/#method-search>
pub async fn search(query: &Query<'_>) -> Result<Vec<BookmarkTreeNode>, Error> {
    let js_query = js_from_serde(query)?;
    let js_value = sys::chrome().bookmarks().search(&js_query).await;
    serde_from_js(js_value)
}

/// <https://developer.chrome.com/docs/extensions/reference/bookmarks/#type-search-query>
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query<'a> {
    /// A string of words and quoted phrases that are matched against bookmark URLs and titles.
    pub query: Option<&'a str>,

    /// The title of the bookmark; matches verbatim.
    pub title: Option<&'a str>,

    /// The URL of the bookmark; matches verbatim. Note that folders have no URL.
    pub url: Option<&'a str>,
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(q: &'a str) -> Self {
        Self {
            query: Some(q),
            title: None,
            url: None,
        }
    }
}

/// <https://developer.chrome.com/docs/extensions/reference/bookmarks/#type-BookmarkTreeNode>
///
/// A node (either a bookmark or a folder) in the bookmark tree.
/// Child nodes are ordered within their parent folder.

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkTreeNode {
    /// Unique identifier.
    pub id: String,

    /// An ordered list of children of this node.
    pub children: Option<Vec<Self>>,

    /// The 0-based position of this node within its parent folder.
    pub index: Option<u32>,

    /// A string which specifies the ID of the parent folder. This property is
    /// not present in the root node.
    pub parent_id: Option<String>,

    /// Date and time of the creation of the bookmark.
    ///
    /// Unix time as milliseconds since the epoch.
    pub date_added: Option<i64>,

    /// When the contents of this folder last changed, in milliseconds since the epoch.
    pub date_group_modified: Option<i64>,

    /// The text displayed for the node in menus and lists of bookmarks.
    pub title: String,

    /// The URL for the bookmark. Empty if this node is a Folder.
    pub url: Option<String>,
}
