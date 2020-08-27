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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::cmp::PartialEq;
    use std::fmt::Debug;

    fn assert_json_serialize_eq<'a, T>(left: &'a T, right: &'a str)
    where
        T: serde::Serialize + Debug,
    {
        assert_eq!(
            serde_json::to_string(left).expect(&format!("failed to serialize {:?} to JSON", left)),
            right
        );
    }

    fn assert_json_deserialize_eq<'a, T>(left: &'a str, right: &'a T)
    where
        T: serde::Deserialize<'a> + PartialEq + Debug,
    {
        assert_eq!(
            &serde_json::from_str::<T>(left)
                .expect(&format!("failed to deserialize JSON {}", left)),
            right
        )
    }

    fn assert_json_serde_eq<'a, T>(left: &'a T, right: &'a str)
    where
        T: serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug,
    {
        assert_json_serialize_eq(left, right);
        assert_json_deserialize_eq(right, left);
    }

    struct JSONSerdeTestCase<'a, T> {
        value: T,
        json: &'a str,
    }

    fn assert_json_serde_test_cases<'a, T, I>(tcs: I)
    where
        T: 'a + serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug,
        I: 'a + IntoIterator<Item = &'a JSONSerdeTestCase<'a, T>>,
    {
        for tc in tcs {
            assert_json_serde_eq(&tc.value, tc.json);
        }
    }

    #[test]
    fn color_serde() {
        assert_json_serde_test_cases(&[
            JSONSerdeTestCase {
                value: Color::Blue,
                json: r#""blue""#,
            },
            JSONSerdeTestCase {
                value: Color::Turquoise,
                json: r#""turquoise""#,
            },
            JSONSerdeTestCase {
                value: Color::Green,
                json: r#""green""#,
            },
            JSONSerdeTestCase {
                value: Color::Yellow,
                json: r#""yellow""#,
            },
            JSONSerdeTestCase {
                value: Color::Orange,
                json: r#""orange""#,
            },
            JSONSerdeTestCase {
                value: Color::Red,
                json: r#""red""#,
            },
            JSONSerdeTestCase {
                value: Color::Pink,
                json: r#""pink""#,
            },
            JSONSerdeTestCase {
                value: Color::Purple,
                json: r#""purple""#,
            },
            JSONSerdeTestCase {
                value: Color::Toolbar,
                json: r#""toolbar""#,
            },
        ])
    }

    #[test]
    fn icon_serde() {
        assert_json_serde_test_cases(&[
            JSONSerdeTestCase {
                value: Icon::Fingerprint,
                json: r#""fingerprint""#,
            },
            JSONSerdeTestCase {
                value: Icon::Briefcase,
                json: r#""briefcase""#,
            },
            JSONSerdeTestCase {
                value: Icon::Dollar,
                json: r#""dollar""#,
            },
            JSONSerdeTestCase {
                value: Icon::Cart,
                json: r#""cart""#,
            },
            JSONSerdeTestCase {
                value: Icon::Circle,
                json: r#""circle""#,
            },
            JSONSerdeTestCase {
                value: Icon::Gift,
                json: r#""gift""#,
            },
            JSONSerdeTestCase {
                value: Icon::Vacation,
                json: r#""vacation""#,
            },
            JSONSerdeTestCase {
                value: Icon::Food,
                json: r#""food""#,
            },
            JSONSerdeTestCase {
                value: Icon::Fruit,
                json: r#""fruit""#,
            },
            JSONSerdeTestCase {
                value: Icon::Pet,
                json: r#""pet""#,
            },
            JSONSerdeTestCase {
                value: Icon::Tree,
                json: r#""tree""#,
            },
            JSONSerdeTestCase {
                value: Icon::Chill,
                json: r#""chill""#,
            },
            JSONSerdeTestCase {
                value: Icon::Fence,
                json: r#""fence""#,
            },
        ])
    }
}
