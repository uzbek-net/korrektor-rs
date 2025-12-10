//! Error type for korrektor.
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum KorrektorError {
    InvalidChar(char),
    InvalidNumber(String, String),
    NumberOverflow(String, String),
}

impl Display for KorrektorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KorrektorError::InvalidChar(char) =>
                write!(f, "Invalid character: \"{char}\"! Only Latin and Cyrillic alphabets for Uzbek language are supported."),
            KorrektorError::InvalidNumber(number, message) =>
                write!(f, "{message}: {number}!"),
            KorrektorError::NumberOverflow(number, message) =>
                write!(f, "{message}: {number}")
        }
    }
}
