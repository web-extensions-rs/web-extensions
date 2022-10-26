mod error;
mod event_listener;
mod util;

pub use crate::error::*;

pub mod bookmarks;
pub mod downloads;
pub mod history;
pub mod tabs;

#[cfg(feature = "firefox")]
pub mod contextual_identities;
