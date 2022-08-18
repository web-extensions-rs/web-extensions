use thiserror::Error;
use wasm_bindgen::{convert::FromWasmAbi, describe::WasmDescribe, prelude::*};

#[derive(Debug, Error)]
pub enum Error {
    #[error("JavaScript error: {0:?}")]
    Js(JsValue),
    #[error(transparent)]
    JsonDeserialization(serde_json::Error),
    #[error(transparent)]
    JsonSerialization(serde_json::Error),
    #[error("Unable to convert JS value to an JS object")]
    ObjectConversion,
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Self::Js(err)
    }
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
