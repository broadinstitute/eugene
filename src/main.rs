use eugene::error::Error;
use eugene::endpoints::{lookup, xrefs};
use eugene::util;
use crate::cli::{Config, LookupConfig, UtilConfig, XrefsConfig};

mod cli;

fn main() {
    match run() {
        Ok(_) => {}
        Err(error) => { eprintln!("Error: {error}"); }
    }
}

fn run() -> Result<(), Error> {
    match cli::get_config()? {
        Config::Xrefs(xrefs_config) => { xrefs(xrefs_config) }
        Config::Lookup(lookup_config) => { lookup(lookup_config) }
        Config::Util(util_config) => { util(util_config) }
    }
}

fn xrefs(xrefs_config: XrefsConfig) -> Result<(), Error> {
    match xrefs_config {
        XrefsConfig::Symbol { species, symbol } => {
            for entry in xrefs::symbol(species, symbol)? {
                println!("{}\t{}", entry.id, entry.tpe)
            }
            Ok(())
        }
    }
}

fn lookup(lookup_config: LookupConfig) -> Result<(), Error> {
    match lookup_config {
        LookupConfig::Symbol { species, symbol } => {
            let entry = lookup::symbol(species, symbol)?;
            println!("{}", entry.id);
            Ok(())
        }
        LookupConfig::Symbols { species, symbols } => {
            todo!()
        }
    }
}

fn util(util_config: UtilConfig) -> Result<(), Error> {
    match util_config {
        UtilConfig::SymbolToGeneId { species, symbol } => {
            let gene_id = util::symbol_to_gene_id(species, symbol)?;
            println!("{gene_id}");
            Ok(())
        }
    }
}
