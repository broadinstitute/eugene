use std::collections::HashMap;
use crate::endpoints::lookup;
use crate::error::Error;
use crate::model::{GeneId, Species, Symbol};

pub fn symbol_to_gene_id(species: Species, symbol: Symbol) -> Result<GeneId, Error> {
    Ok(lookup::symbol(species, symbol)?.id)
}

pub fn symbols_to_gene_ids(species: Species, symbols: Vec<Symbol>)
    -> Result<HashMap<Symbol, GeneId>, Error> {
    let mut entries = lookup::symbols(species, symbols)?;
    let ids: HashMap<Symbol, GeneId> =
        entries.drain().map(|(symbol, entry)| (symbol, entry.id)).collect();
    Ok(ids)
}