//! Error types for wtvr3d

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    WebGlError,
    UniformError,
    UnconstructedValue,
    MisingData,
    UnknownLocation,
    UnknownTextureNumber,
    MisingAsset,
    Unimplemented,
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let err_msg = match self {
            Error::WebGlError => "A WebGl Operation Failed.",
            Error::UniformError => "An uniform could not be set.",
            Error::UnconstructedValue => "Trying to use a raw, unconstructed value.",
            Error::MisingData => "Trying to use a missing asset.",
            Error::UnknownLocation => "Trying to bind an attribute or uniform of unknown location",
            Error::UnknownTextureNumber => {
                "Trying to bind a texture uniform without a texture number"
            }
            Error::MisingAsset => "Trying to use a missing asset.",
            Error::Unimplemented => "Trying to use unimplemented feature.",
            Error::Unknown => "An unknown error has occured.",
        };
        f.write_str(&err_msg)
    }
}