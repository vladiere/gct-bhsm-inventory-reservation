// region: Error

use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[derive(Debug, Serialize)]
pub enum Error {
    Key,
    Salt,
    Hash,
    PassValidate,
    SchemeNotFound(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error boilerplate

// endregion: Error
