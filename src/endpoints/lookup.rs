use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::model::{GeneId, Species, Symbol};
use crate::endpoints::BASE_URL;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: GeneId
}

#[derive(Serialize, Deserialize)]
pub struct Symbols {
    symbols: Vec<Symbol>
}

pub fn symbol(species: Species, symbol: Symbol) -> Result<Entry, Error> {
    let url =
        format!("{BASE_URL}/lookup/symbol/{species}/{symbol}?content-type=application/json");
    let response = reqwest::blocking::get(url)?;
    Ok(response.json::<Entry>()?)
}

pub fn symbols(species: Species, symbols: Vec<Symbol>) -> Result<HashMap<Symbol, Entry>, Error> {
    let url = format!("{BASE_URL}/lookup/symbol/{species}?content-type=application/json");
    let symbols = Symbols { symbols };
    let client = reqwest::blocking::Client::new();
    let response = client.post(url).json(&symbols).send()?;
    Ok(response.json::<HashMap<Symbol, Entry>>()?)
}