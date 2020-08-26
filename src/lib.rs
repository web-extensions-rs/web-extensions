use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum Error {
    JsError(JsValue),
    JSONError(serde_json::Error),
}

impl From<JsValue> for Error {
    fn from(v: JsValue) -> Self {
        Self::JsError(v)
    }
}

impl From<serde_json::Error> for Error {
    fn from(v: serde_json::Error) -> Self {
        Self::JSONError(v)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
struct CreateDetails<'a> {
    name: Option<&'a str>,
    color: Option<Color>,
    icon: Option<Icon>,
}

pub async fn create(
    name: Option<&str>,
    color: Option<Color>,
    icon: Option<Icon>,
) -> Result<ContextualIdentity, Error> {
    contextual_identities_sys::create(&JsValue::from_serde(&CreateDetails { name, color, icon })?)
        .await?
        .into_serde::<ContextualIdentity>()
        .map_err(From::from)
}

#[derive(Serialize)]
struct UpdateDetails<'a> {
    name: Option<&'a str>,
    color: Option<Color>,
    icon: Option<Icon>,
}

pub async fn update(
    cookie_store_id: &str,
    name: Option<&str>,
    color: Option<Color>,
    icon: Option<Icon>,
) -> Result<ContextualIdentity, Error> {
    contextual_identities_sys::update(
        cookie_store_id,
        &JsValue::from_serde(&UpdateDetails { name, color, icon })?,
    )
    .await?
    .into_serde::<ContextualIdentity>()
    .map_err(From::from)
}

#[derive(Serialize)]
struct QueryDetails<'a> {
    name: Option<&'a str>,
}

pub async fn query(name: Option<&str>) -> Result<Vec<ContextualIdentity>, Error> {
    contextual_identities_sys::query(&JsValue::from_serde(&QueryDetails { name })?)
        .await?
        .into_serde::<Vec<ContextualIdentity>>()
        .map_err(From::from)
}
