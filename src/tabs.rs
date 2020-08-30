use crate::{
    js_from_serde, object_from_js, serde_from_js_result, Error, EventTarget,
};

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;
use web_extensions_sys::{browser, Tabs};

fn tabs() -> Tabs {
    browser.tabs()
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename(serialize = "loading", deserialize = "loading"))]
    Loading,
    #[serde(rename(serialize = "complete", deserialize = "complete"))]
    Complete,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum WindowType {
    #[serde(rename(serialize = "normal"))]
    Normal,
    #[serde(rename(serialize = "popup"))]
    Popup,
    #[serde(rename(serialize = "panel"))]
    Panel,
    #[serde(rename(serialize = "devtools"))]
    Devtools,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub active: bool,
    pub attention: Option<bool>,
    pub audible: Option<bool>,
    pub auto_discardable: Option<bool>,
    pub cookie_store_id: Option<String>,
    pub discarded: Option<bool>,
    pub fav_icon_url: Option<String>,
    pub height: Option<i32>,
    pub hidden: bool,
    pub highlighted: bool,
    pub id: Option<i32>,
    pub incognito: bool,
    pub index: i32,
    pub is_article: bool,
    pub is_in_reader_mode: bool,
    pub last_accessed: f64,
    // TODO: muted_info
    pub opener_tab_id: Option<i32>,
    pub pinned: bool,
    pub session_id: Option<String>,
    pub status: Option<Status>,
    pub successor_tab_id: Option<i32>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub width: Option<i32>,
    pub window_id: i32,
}

pub async fn get(tab_id: i32) -> Result<Tab, Error> {
    serde_from_js_result(tabs().get(tab_id).await)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDetails<'a> {
    pub active: Option<bool>,
    pub audible: Option<bool>,
    pub auto_discardable: Option<bool>,
    pub cookie_store_id: Option<&'a str>,
    pub current_window: Option<bool>,
    pub discarded: Option<bool>,
    pub hidden: Option<bool>,
    pub highlighted: Option<bool>,
    pub index: Option<i32>,
    pub muted: Option<bool>,
    pub last_focused_window: Option<bool>,
    pub pinned: Option<bool>,
    pub status: Option<Status>,
    pub title: Option<&'a str>,
    pub url: Option<&'a str>,
    pub window_id: i32,
    pub window_type: Option<WindowType>,
}

pub async fn query(details: &QueryDetails<'_>) -> Result<Vec<Tab>, Error> {
    serde_from_js_result(
        tabs()
            .query(object_from_js(&js_from_serde(details)?)?)
            .await,
    )
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveInfo {
    pub previous_tab_id: i32,
    pub tab_id: i32,
    pub window_id: i32,
}

pub fn on_activated() -> EventTarget<ActiveInfo> {
    EventTarget(tabs().on_activated(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachInfo {
    pub new_window_id: i32,
    pub new_position: i32,
}

pub fn on_attached() -> EventTarget<(i32, AttachInfo)> {
    EventTarget(tabs().on_attached(), PhantomData)
}

pub fn on_created() -> EventTarget<Tab> {
    EventTarget(tabs().on_created(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetachInfo {
    pub old_window_id: i32,
    pub old_position: i32,
}

pub fn on_detached() -> EventTarget<(i32, DetachInfo)> {
    EventTarget(tabs().on_detached(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightInfo {
    pub window_id: i32,
    pub tab_ids: Vec<i32>,
}

pub fn on_highlighted() -> EventTarget<HighlightInfo> {
    EventTarget(tabs().on_highlighted(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveInfo {
    pub window_id: i32,
    pub from_index: i32,
    pub to_index: i32,
}

pub fn on_moved() -> EventTarget<(i32, MoveInfo)> {
    EventTarget(tabs().on_moved(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInfo {
    pub window_id: i32,
    pub is_window_closing: bool,
}

pub fn on_removed() -> EventTarget<(i32, RemoveInfo)> {
    EventTarget(tabs().on_removed(), PhantomData)
}

pub fn on_replaced() -> EventTarget<(i32, i32)> {
    EventTarget(tabs().on_replaced(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInfo {
    // TODO: Add more fields from https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/onUpdated#changeInfo
    pub url: Option<String>,
}

pub fn on_updated() -> EventTarget<(i32, ChangeInfo, Tab)> {
    EventTarget(tabs().on_updated(), PhantomData)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoomChangeInfo {
    // TODO: Add more fields from https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/onZoomChange#ZoomChangeInfo
}

pub fn on_zoom_change() -> EventTarget<ZoomChangeInfo> {
    EventTarget(tabs().on_zoom_change(), PhantomData)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn status_serde() {
        assert_json_serde_test_cases(&[
            JSONSerdeTestCase {
                value: Status::Loading,
                json: r#""loading""#,
            },
            JSONSerdeTestCase {
                value: Status::Complete,
                json: r#""complete""#,
            },
        ])
    }

    #[test]
    fn window_type_serialize() {
        assert_json_serialize_test_cases(&[
            JSONSerdeTestCase {
                value: WindowType::Normal,
                json: r#""normal""#,
            },
            JSONSerdeTestCase {
                value: WindowType::Popup,
                json: r#""popup""#,
            },
            JSONSerdeTestCase {
                value: WindowType::Panel,
                json: r#""panel""#,
            },
            JSONSerdeTestCase {
                value: WindowType::Devtools,
                json: r#""devtools""#,
            },
        ])
    }
}
