use hex;
use std::fmt;

use keys::error as KeyError;

use crate::{ParseAssetError, ParseNameError, ParseSymbolError, ReadError, WriteError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    CustomError(String),
    BytesReadError(ReadError),
    BytesWriteError(WriteError),
    FromHexError(hex::FromHexError),
    Keys(KeyError::Error),
    ParseAssetErr(ParseAssetError),
    ParseNameErr(ParseNameError),
    ParseSymbolError(ParseSymbolError),
    FromTrxKindsError,
}
