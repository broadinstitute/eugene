use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::model::{Gene, Species, Symbol};
use crate::endpoints::BASE_URL;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "type")]
    pub tpe: String,
    pub id: String
}

pub fn symbol(species: Species, symbol: Symbol) -> Result<Vec<Entry>, Error> {
    let url =
        format!("{BASE_URL}/xrefs/symbol/{species}/{symbol}?content-type=application/json");
    let response = reqwest::blocking::get(url)?;
    Ok(response.json::<Vec<Entry>>()?)
}

pub fn symbol_to_gene(species: Species, symbol: Symbol) -> Result<Gene, Error> {
    let url =
        format!("{BASE_URL}/xrefs/symbol/{species}/{symbol}?content-type=application/json");
    let response = reqwest::blocking::get(url)?;
    println!("{}", response.text()?);  // TODO
    Gene::try_from("HELLO")  //  TODO
}