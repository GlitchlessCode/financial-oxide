use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct Charge {
    dollars: u32,
    cents: u8,
}

pub struct ChargeConversionError {
    src: String,
}

impl Display for ChargeConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Debug for ChargeConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for ChargeConversionError {}

impl TryFrom<String> for Charge {
    type Error = ChargeConversionError;
    fn try_from(src: String) -> Result<Self, Self::Error> {
        Err(ChargeConversionError { src })
    }
}
