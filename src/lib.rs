#![feature(unsize)]
#![feature(unboxed_closures)]

use js_sys::Object;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::marker::Unsize;
use wasm_bindgen::closure::WasmClosure;
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
