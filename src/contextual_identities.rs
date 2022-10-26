//! Wrapper for the [`browser.contextualIdentities` API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/contextualIdentities).

use crate::{
    util::{js_from_serde, object_from_js, serde_from_js_result},
    Error,
};

use serde::{Deserialize, Serialize};
use web_extensions_sys::{browser, ContextualIdentities};

fn contextual_identities() -> ContextualIdentities {
    browser.contextual_identities()
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename(serialize = "blue", deserialize = "blue"))]
    Blue,
    #[serde(rename(serialize = "turquoise", deserialize = "turquoise"))]
    Turquoise,
    #[serde(rename(serialize = "green", deserialize = "green"))]
    Green,
    #[serde(rename(serialize = "yellow", deserialize = "yellow"))]
    Yellow,
    #[serde(rename(serialize = "orange", deserialize = "orange"))]
    Orange,
    #[serde(rename(serialize = "red", deserialize = "red"))]
    Red,
    #[serde(rename(serialize = "pink", deserialize = "pink"))]
    Pink,
    #[serde(rename(serialize = "purple", deserialize = "purple"))]
    Purple,
    #[serde(rename(serialize = "toolbar", deserialize = "toolbar"))]
    Toolbar,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename(serialize = "fingerprint", deserialize = "fingerprint"))]
    Fingerprint,
    #[serde(rename(serialize = "briefcase", deserialize = "briefcase"))]
    Briefcase,
    #[serde(rename(serialize = "dollar", deserialize = "dollar"))]
    Dollar,
    #[serde(rename(serialize = "cart", deserialize = "cart"))]
    Cart,
    #[serde(rename(serialize = "circle", deserialize = "circle"))]
    Circle,
    #[serde(rename(serialize = "gift", deserialize = "gift"))]
    Gift,
    #[serde(rename(serialize = "vacation", deserialize = "vacation"))]
    Vacation,
    #[serde(rename(serialize = "food", deserialize = "food"))]
    Food,
    #[serde(rename(serialize = "fruit", deserialize = "fruit"))]
    Fruit,
    #[serde(rename(serialize = "pet", deserialize = "pet"))]
    Pet,
    #[serde(rename(serialize = "tree", deserialize = "tree"))]
    Tree,
    #[serde(rename(serialize = "chill", deserialize = "chill"))]
    Chill,
    #[serde(rename(serialize = "fence", deserialize = "fence"))]
    Fence,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextualIdentity {
    pub cookie_store_id: String,
    pub color: Color,
    pub color_code: String,
    pub icon: Icon,
    pub icon_url: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateDetails<'a> {
    pub name: &'a str,
    pub color: Color,
    pub icon: Icon,
}

pub async fn create(details: &CreateDetails<'_>) -> Result<ContextualIdentity, Error> {
    serde_from_js_result(
        contextual_identities()
            .create(object_from_js(&js_from_serde(details)?)?)
            .await,
    )
}

pub async fn get(cookie_store_id: &str) -> Result<ContextualIdentity, Error> {
    serde_from_js_result(contextual_identities().get(cookie_store_id).await)
}

#[derive(Serialize)]
pub struct QueryDetails<'a> {
    pub name: Option<&'a str>,
}

pub async fn query(details: &QueryDetails<'_>) -> Result<Vec<ContextualIdentity>, Error> {
    serde_from_js_result(
        contextual_identities()
            .query(object_from_js(&js_from_serde(details)?)?)
            .await,
    )
}

pub async fn remove(cookie_store_id: &str) -> Result<ContextualIdentity, Error> {
    serde_from_js_result(contextual_identities().remove(cookie_store_id).await)
}

#[derive(Serialize)]
pub struct UpdateDetails<'a> {
    pub name: Option<&'a str>,
    pub color: Option<Color>,
    pub icon: Option<Icon>,
}

pub async fn update(
    cookie_store_id: &str,
    details: &UpdateDetails<'_>,
) -> Result<ContextualIdentity, Error> {
    serde_from_js_result(
        contextual_identities()
            .update(cookie_store_id, object_from_js(&js_from_serde(details)?)?)
            .await,
    )
}
