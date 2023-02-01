use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::model::{GeneId, Species, Symbol};
use crate::endpoints::BASE_URL;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: GeneId
}

pub fn symbol(species: Species, symbol: Symbol) -> Result<Entry, Error> {
    let url =
        format!("{BASE_URL}/lookup/symbol/{species}/{symbol}?content-type=application/json");
    let response = reqwest::blocking::get(url)?;
    Ok(response.json::<Entry>()?)
}