use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Species(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneId(String);

impl TryFrom<&str> for Species {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Species::try_from(string.to_string())
    }
}

impl TryFrom<String> for Species {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Ok(Species(string))
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Symbol {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Symbol::try_from(string.to_string())
    }
}

impl TryFrom<String> for Symbol {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> { Ok(Symbol(string)) }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl TryFrom<&str> for GeneId {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        GeneId::try_from(string.to_string())
    }
}

impl TryFrom<String> for GeneId {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> { Ok(GeneId(string)) }
}

impl Display for GeneId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}