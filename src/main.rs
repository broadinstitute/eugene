use eugene::error::Error;
use crate::cli::Config;

mod cli;

fn main() -> Result<(), Error>{
    match cli::get_config() {
        Ok(config) => {
            match config {
                Config::SymbolToGene(symbol_to_gene) => {
                    let gene =
                        eugene::symbol_to_gene(symbol_to_gene.species, symbol_to_gene.symbol)?;
                    println!("{gene}");
                    Ok(())
                }
            }
        }
        Err(error) => {
            eprintln!("{error}");
            Err(error)
        }
    }
}
