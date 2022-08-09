use js_sys::Object;
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::{convert::FromWasmAbi, describe::WasmDescribe, prelude::*};

#[cfg(test)]
mod test_util;

mod event_listener;
pub use event_listener::*;

pub mod contextual_identities;
pub mod tabs;

pub struct EventTarget<A>(web_extensions_sys::EventTarget, PhantomData<A>);

#[derive(Debug)]
pub enum Error {
    JsError(JsValue),
    JSONDeserializationError(serde_json::Error),
    JSONSerializationError(serde_json::Error),
    ObjectConversionError,
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Self::JsError(err)
    }
}

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

#[derive(Debug)]
pub enum FromWasmAbiResult<T, E> {
    /// Contains the success value
    Ok(T),

    /// Contains the error value
    Err(E),
}

impl<T, E> From<FromWasmAbiResult<T, E>> for Result<T, E> {
    fn from(result: FromWasmAbiResult<T, E>) -> Self {
        match result {
            FromWasmAbiResult::Ok(v) => Ok(v),
            FromWasmAbiResult::Err(e) => Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for FromWasmAbiResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
}

pub type SerdeFromWasmAbiResult<T> = FromWasmAbiResult<T, serde_json::Error>;

impl<T> WasmDescribe for SerdeFromWasmAbiResult<T> {
    #[inline]
    fn describe() {
        JsValue::describe()
    }
}
impl<T: for<'a> serde::Deserialize<'a>> FromWasmAbi for SerdeFromWasmAbiResult<T> {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> Self {
        JsValue::from_abi(js).into_serde().into()
    }
}
