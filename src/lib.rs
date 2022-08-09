use js_sys::Object;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

mod error;
mod event_listener;

pub use crate::{error::*, event_listener::*};

pub mod contextual_identities;
pub mod tabs;

#[cfg(test)]
mod test_util;

pub struct EventTarget<A>(web_extensions_sys::EventTarget, PhantomData<A>);

fn js_from_serde<T: serde::Serialize>(v: &T) -> Result<JsValue, Error> {
    JsValue::from_serde(v).map_err(Error::JSONSerializationError)
}

fn object_from_js(v: &JsValue) -> Result<&Object, Error> {
    Object::try_from(v).ok_or(Error::ObjectConversionError)
}

fn serde_from_js_result<T>(v: Result<JsValue, JsValue>) -> Result<T, Error>
where
    T: for<'a> serde::Deserialize<'a>,
{
    v?.into_serde().map_err(Error::JSONDeserializationError)
}
