use std::marker::PhantomData;

mod error;
mod event_listener;
mod util;

pub use crate::{error::*, event_listener::*};

pub mod contextual_identities;
pub mod tabs;

#[cfg(test)]
mod test_util;

pub struct EventTarget<A>(web_extensions_sys::EventTarget, PhantomData<A>);
