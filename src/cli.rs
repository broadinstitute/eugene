use clap::{Arg, command, Command};
use eugene::error::Error;
use eugene::model::{Species, Symbol};

pub(crate) enum Config {
    SymbolToGene(SymbolToGene)
}

pub(crate) struct SymbolToGene {
    pub(crate) species: Species,
    pub(crate) symbol: Symbol,
}

const SYMBOL_TO_GENE: &str = "symbol-to-gene";
const SPECIES: &str = "species";
const HOMO_SAPIENS: &str = "homo_sapiens";
const SYMBOL: &str = "symbol";

pub(crate) fn get_config() -> Result<Config, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(SYMBOL_TO_GENE)
                .about("Map symbol to gene id")
                .arg(Arg::new("species")
                    .short('p')
                    .long("species")
                    .required(false))
                .arg(Arg::new("symbol")
                    .short('s')
                    .long("symbol"))
        ).get_matches();
    match matches.subcommand() {
        Some((SYMBOL_TO_GENE, sub_matches)) => {
            let species =
                Species::parse_str(sub_matches.get_one::<String>(SPECIES)
                    .unwrap_or(&HOMO_SAPIENS.to_string()))?;
            let symbol =
                Symbol::parse_str(
                    sub_matches.get_one::<String>(SYMBOL)
                        .ok_or_else(|| {
                            Error::from(format!("Missing argument {SYMBOL}"))
                        })?
                )?;
            Ok(Config::SymbolToGene(SymbolToGene { species, symbol }))
        }
        Some((unknown_command, _)) => {
            Err(Error::from(format!(
                "Unknown command {unknown_command}. Known command is {SYMBOL_TO_GENE}"
            )))
        }
        None => {
            Err(Error::from(
                format!("Command required. Known command is {SYMBOL_TO_GENE}")
            ))
        }
    }
}