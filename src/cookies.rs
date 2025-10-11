use serde::{Deserialize, Serialize};

use crate::util::{js_from_serde, object_from_js, serde_from_js};

pub async fn get(details: CookieDetails) -> Option<Cookie> {
    let js_query = js_from_serde(&details).unwrap();
    let js_query = object_from_js(&js_query).unwrap();
    let js_value = web_extensions_sys::chrome().cookies().get(js_query).await;
    serde_from_js(js_value).unwrap()
}

pub async fn set(details: SetCookieDetails) -> Option<Cookie> {
    let js_query = js_from_serde(&details).unwrap();
    let js_query = object_from_js(&js_query).unwrap();
    let js_value = web_extensions_sys::chrome().cookies().set(js_query).await;
    serde_from_js(js_value).unwrap()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieDetails {
    pub domain: Option<String>,
    pub expiration_date: Option<u64>,
    pub http_only: Option<bool>,
    pub name: Option<String>,
    pub partition_key: Option<CookiePartitionKey>,
    pub path: Option<String>,
    pub same_site: Option<SameSiteStatus>,
    pub secure: Option<bool>,
    pub store_id: Option<String>,
    pub url: String,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieDetails {
    pub name: String,
    pub partition_key: Option<CookiePartitionKey>,
    pub store_id: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookiePartitionKey {
    pub has_cross_site_ancestor: Option<bool>,
    pub top_level_site: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub domain: String,
    pub expiration_date: Option<u64>,
    pub host_only: bool,
    pub http_only: bool,
    pub name: String,
    pub partition_key: Option<CookiePartitionKey>,
    pub path: String,
    pub same_site: SameSiteStatus,
    pub secure: bool,
    pub session: bool,
    pub store_id: String,
    pub value: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SameSiteStatus {
    NoRestriction,
    Lax,
    Strict,
    Unspecified,
}
