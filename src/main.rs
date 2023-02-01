use eugene::error::Error;
use eugene::endpoints::xrefs;
use crate::cli::{Config, XrefsConfig};

mod cli;

fn main() -> Result<(), Error> {
    match cli::get_config() {
        Ok(config) => {
            match config {
                Config::Xrefs(xrefs_config) => { xrefs(xrefs_config) }
            }
        }
        Err(error) => {
            eprintln!("{error}");
            Err(error)
        }
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
