use serde::{Deserialize, Serialize};

use super::RawString;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum Formatter {
    String(String),
    Function(RawString),
}

impl From<&str> for Formatter {
    fn from(s: &str) -> Self {
        Formatter::String(s.to_string())
    }
}

impl From<RawString> for Formatter {
    fn from(s: RawString) -> Self {
        Formatter::Function(s)
    }
}
