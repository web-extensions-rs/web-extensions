use super::prelude::*;
use thiserror::Error;

/// <https://developer.chrome.com/docs/extensions/reference/tabs/#type-TabStatus>
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename(serialize = "unloaded", deserialize = "unloaded"))]
    Unloaded,
    #[serde(rename(serialize = "loading", deserialize = "loading"))]
    Loading,
    #[serde(rename(serialize = "complete", deserialize = "complete"))]
    Complete,
}

#[derive(Debug, Error)]
#[error("Invalid status ('{0}'), expected 'unloaded', 'loading' or 'complete'")]
pub struct InvalidStatusError(String);

impl TryFrom<String> for Status {
    type Error = InvalidStatusError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match &*s {
            "unloaded" => Ok(Status::Unloaded),
            "loading" => Ok(Status::Loading),
            "complete" => Ok(Status::Complete),
            _ => Err(InvalidStatusError(s)),
        }
    }
}
