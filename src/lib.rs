#![feature(unsize)]
#![feature(unboxed_closures)]

use js_sys::Object;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::marker::Unsize;
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

pub mod contextual_identities;
pub mod tabs;

mod event_listener;

pub use event_listener::*;

pub struct EventTarget<A>(web_extensions_sys::EventTarget, PhantomData<A>);

impl<A> EventTarget<A> {
    pub fn add_listener<L, W>(&self, listener: L) -> EventListener<W>
    where
        W: ?Sized + WasmClosure + FnMut<A>,
        L: FnMut<A> + Unsize<W> + 'static,
    {
        EventListener::new(&self.0, listener)
    }
}

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

#[cfg(test)]
pub mod test_util {
    use std::cmp::PartialEq;
    use std::fmt::Debug;

    pub fn assert_json_serialize_eq<'a, T>(left: &'a T, right: &'a str)
    where
        T: serde::Serialize + Debug,
    {
        assert_eq!(
            serde_json::to_string(left).expect(&format!("failed to serialize {:?} to JSON", left)),
            right
        );
    }

    pub fn assert_json_deserialize_eq<'a, T>(left: &'a str, right: &'a T)
    where
        T: serde::Deserialize<'a> + PartialEq + Debug,
    {
        assert_eq!(
            &serde_json::from_str::<T>(left)
                .expect(&format!("failed to deserialize JSON {}", left)),
            right
        )
    }

    pub fn assert_json_serde_eq<'a, T>(left: &'a T, right: &'a str)
    where
        T: serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug,
    {
        assert_json_serialize_eq(left, right);
        assert_json_deserialize_eq(right, left);
    }

    pub struct JSONSerdeTestCase<'a, T> {
        pub value: T,
        pub json: &'a str,
    }

    pub fn assert_json_serialize_test_cases<'a, T, I>(tcs: I)
    where
        T: 'a + serde::Serialize + PartialEq + Debug,
        I: 'a + IntoIterator<Item = &'a JSONSerdeTestCase<'a, T>>,
    {
        for tc in tcs {
            assert_json_serialize_eq(&tc.value, tc.json);
        }
    }

    pub fn assert_json_deserialize_test_cases<'a, T, I>(tcs: I)
    where
        T: 'a + serde::Deserialize<'a> + PartialEq + Debug,
        I: 'a + IntoIterator<Item = &'a JSONSerdeTestCase<'a, T>>,
    {
        for tc in tcs {
            assert_json_deserialize_eq(tc.json, &tc.value);
        }
    }

    pub fn assert_json_serde_test_cases<'a, T, I>(tcs: I)
    where
        T: 'a + serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug,
        I: 'a + IntoIterator<Item = &'a JSONSerdeTestCase<'a, T>>,
    {
        for tc in tcs {
            assert_json_serde_eq(&tc.value, tc.json);
        }
    }
}
