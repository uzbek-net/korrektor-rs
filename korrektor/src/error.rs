use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum KorrektorError {
    InvalidChar(char)
}

impl Display for KorrektorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KorrektorError::InvalidChar(char) =>
                write!(f, "Invalid character: \"{char}\"! Only Latin and Cyrillic alphabets for Uzbek language are supported.")
        }
    }
}