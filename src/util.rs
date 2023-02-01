use crate::endpoints::lookup;
use crate::error::Error;
use crate::model::{GeneId, Species, Symbol};

pub fn symbol_to_gene_id(species: Species, symbol: Symbol) -> Result<GeneId, Error> {
    Ok(lookup::symbol(species, symbol)?.id)
}