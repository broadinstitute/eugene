use std::fmt::{Display, Formatter};
use crate::error::Error;

#[derive(Debug)]
pub struct Species {
    string: String,
}

#[derive(Debug)]
pub struct Symbol {
    string: String,
}

#[derive(Debug)]
pub struct Gene {
    string: String,
}

impl Species {
    pub fn parse_str(string: &str) -> Result<Species, Error> {
        Species::parse(string.to_string())
    }
    pub fn parse(string: String) -> Result<Species, Error> {
        Ok(Species { string })
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Symbol {
    pub fn parse_str(string: &str) -> Result<Symbol, Error> {
        Symbol::parse(string.to_string())
    }
    pub fn parse(string: String) -> Result<Symbol, Error> {
        Ok(Symbol { string })
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Gene {
    pub fn parse_str(string: &str) -> Result<Gene, Error> {
        Gene::parse(string.to_string())
    }
    pub fn parse(string: String) -> Result<Gene, Error> {
        Ok(Gene { string })
    }
}

impl Display for Gene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}