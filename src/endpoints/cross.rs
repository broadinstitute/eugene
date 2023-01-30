use crate::error::Error;
use crate::model::{Gene, Species, Symbol};
use crate::endpoints::BASE_URL;

pub fn symbol_to_gene(species: Species, symbol: Symbol) -> Result<Gene, Error> {
    let url =
        format!("{BASE_URL}/xrefs/symbol/{species}/{symbol}?content-type=application/json");
    let response = reqwest::blocking::get(url)?;
    println!("{}", response.text()?);  // TODO
    Gene::parse_str("HELLO")  //  TODO
}