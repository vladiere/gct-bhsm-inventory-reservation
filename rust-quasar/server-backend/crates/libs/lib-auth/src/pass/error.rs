// region: --- Error

use derive_more::derive::From;
use serde::Serialize;

use super::scheme;

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[derive(Debug, Serialize, From)]
pub enum Error {
    PassWithSchemeFailedToParse,

    FailSpawnBlockForValidate,
    FailSpawnBlockForHash,

    // --- Modules
    Scheme(scheme::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error boilerplate

// endregion: --- Error
