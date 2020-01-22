use hex;
use keys::error as KeyError;
use crate::{ParseAssetError, ParseNameError, ParseSymbolError, ReadError, WriteError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    FromTrxKindsError,
    CustomError(String),
    Keys(KeyError::Error),
    BytesReadError(ReadError),
    BytesWriteError(WriteError),
    ParseNameErr(ParseNameError),
    FromHexError(hex::FromHexError),
    ParseAssetErr(ParseAssetError),
    ParseSymbolError(ParseSymbolError),
}
