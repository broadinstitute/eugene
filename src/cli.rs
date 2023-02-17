use clap::{command, Command};
use eugene::error::Error;
use eugene::model::{Species, Symbol};

pub(crate) enum Config {
    Xrefs(XrefsConfig),
    Lookup(LookupConfig),
    Util(UtilConfig)
}

pub(crate) enum XrefsConfig {
    Symbol { species: Species, symbol: Symbol }
}

pub(crate) enum LookupConfig {
    Symbol { species: Species, symbol: Symbol },
    Symbols { species: Species, symbols: Vec<Symbol> }
}

pub(crate) enum UtilConfig {
    SymbolToGeneId { species: Species, symbol: Symbol },
    SymbolsToGeneIds { species: Species, symbols: Vec<Symbol> }
}

mod section {
    pub(crate) const XREFS: &str = "xrefs";
    pub(crate) const LOOKUP: &str = "lookup";
    pub(crate) const UTIL: &str = "util";
    pub(crate) const LIST: &[&str] = &[XREFS, LOOKUP, UTIL];
}

mod cmd {
    pub(crate) const SYMBOL: &str = "symbol";
    pub(crate) const SYMBOLS: &str = "symbols";
    pub(crate) const SYMBOL_TO_GENE_ID: &str = "symbol-to-gene-id";
    pub(crate) const SYMBOLS_TO_GENE_IDS: &str = "symbols-to-gene-ids";
    pub(crate) const XREFS_CMDS: &[&str] = &[SYMBOL];
    pub(crate) const LOOKUP_CMDS: &[&str] = &[SYMBOL, SYMBOLS];
    pub(crate) const UTIL_CMDS: &[&str] = &[SYMBOL_TO_GENE_ID, SYMBOLS_TO_GENE_IDS];
}

mod arg {
    use clap::Arg;

    pub(crate) const SPECIES: &str = "species";
    pub(crate) const SYMBOL: &str = "symbol";
    pub(crate) const SYMBOLS: &str = "symbols";

    pub(crate) fn species() -> Arg {
        Arg::new(SPECIES).short('p').long(SPECIES).required(false)
    }

    pub(crate) fn symbol() -> Arg { Arg::new(SYMBOL).short('s').long(SYMBOL) }
    pub(crate) fn symbols() -> Arg { Arg::new(SYMBOLS).short('l').long(SYMBOLS) }
}

mod get {
    use clap::ArgMatches;
    use eugene::error::Error;
    use eugene::model::{Species, Symbol};
    use std::convert::TryFrom;
    use super::arg;

    pub(crate) fn species(matches: &ArgMatches) -> Result<Species, Error> {
        const HOMO_SAPIENS: &str = "homo_sapiens";
        Species::try_from(matches.get_one::<String>(arg::SPECIES).cloned()
            .unwrap_or(HOMO_SAPIENS.to_string()))
    }

    pub(crate) fn symbol(matches: &ArgMatches) -> Result<Symbol, Error> {
        Symbol::try_from(
            matches.get_one::<String>(arg::SYMBOL).cloned()
                .ok_or_else(|| {
                    Error::from(format!("Missing argument {}", arg::SYMBOL))
                })?
        )
    }

    pub(crate) fn symbols(matches: &ArgMatches) -> Result<Vec<Symbol>, Error> {
        let symbols: Result<Vec<Symbol>, Error> =
            matches.get_one::<String>(arg::SYMBOLS).ok_or_else(|| {
                Error::from(format!("Missing argument {}", arg::SYMBOLS))
            })?.as_str().split(',').map(Symbol::try_from)
                .collect();
        symbols
    }
}

fn unknown_cmd_error(cmd: &str, expected: &[&str]) -> Error {
    let message =
        if expected.len() == 1 {
            format!("Unknown command {cmd}. Only known command is {}", expected[0])
        } else {
            format!("Unknown command {cmd}. Known commands are {}", expected.join(", "))
        };
    Error::from(message)
}

fn missing_cmd_error(expected: &[&str]) -> Error {
    let message =
        if expected.len() == 1 {
            format!("Missing command. Only known command is {}", expected[0])
        } else {
            format!("Missing command. Known commands are {}", expected.join(", "))
        };
    Error::from(message)
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(section::XREFS)
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(cmd::SYMBOL)
                        .arg(arg::species())
                        .arg(arg::symbol())
                )
        )
        .subcommand(
            Command::new(section::LOOKUP)
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(cmd::SYMBOL)
                        .arg(arg::species())
                        .arg(arg::symbol())
                )
                .subcommand(
                    Command::new(cmd::SYMBOLS)
                        .arg(arg::species())
                        .arg(arg::symbols())
                )
        )
        .subcommand(
            Command::new(section::UTIL)
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(cmd::SYMBOL_TO_GENE_ID)
                        .arg(arg::species())
                        .arg(arg::symbol())
                )
                .subcommand(
                    Command::new(cmd::SYMBOLS_TO_GENE_IDS)
                        .arg(arg::species())
                        .arg(arg::symbols())
                )
        ).get_matches();
    match matches.subcommand() {
        Some((section::XREFS, section_matches)) => {
            match section_matches.subcommand() {
                Some((cmd::SYMBOL, cmd_matches)) => {
                    let species = get::species(cmd_matches)?;
                    let symbol = get::symbol(cmd_matches)?;
                    Ok(Config::Xrefs(XrefsConfig::Symbol { species, symbol }))
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, cmd::XREFS_CMDS))
                }
                None => {
                    Err(missing_cmd_error(cmd::XREFS_CMDS))
                }
            }
        }
        Some((section::LOOKUP, section_matches)) => {
            match section_matches.subcommand() {
                Some((cmd::SYMBOL, cmd_matches)) => {
                    let species = get::species(cmd_matches)?;
                    let symbol = get::symbol(cmd_matches)?;
                    Ok(Config::Lookup(LookupConfig::Symbol { species, symbol }))
                }
                Some((cmd::SYMBOLS, cmd_matches)) => {
                    let species = get::species(cmd_matches)?;
                    let symbols = get::symbols(cmd_matches)?;
                    Ok(Config::Lookup(LookupConfig::Symbols { species, symbols }))
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, cmd::LOOKUP_CMDS))
                }
                None => {
                    Err(missing_cmd_error(cmd::LOOKUP_CMDS))
                }
            }
        }
        Some((section::UTIL, section_matches)) => {
            match section_matches.subcommand() {
                Some((cmd::SYMBOL_TO_GENE_ID, cmd_matches)) => {
                    let species = get::species(cmd_matches)?;
                    let symbol = get::symbol(cmd_matches)?;
                    Ok(Config::Util(UtilConfig::SymbolToGeneId { species, symbol }))
                }
                Some((cmd::SYMBOLS_TO_GENE_IDS, cmd_matches)) => {
                    let species = get::species(cmd_matches)?;
                    let symbols = get::symbols(cmd_matches)?;
                    Ok(Config::Util(UtilConfig::SymbolsToGeneIds { species, symbols }))
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, cmd::UTIL_CMDS))
                }
                None => {
                    Err(missing_cmd_error(cmd::UTIL_CMDS))
                }
            }
        }
        Some((unknown_cmd, _)) => {
            Err(unknown_cmd_error(unknown_cmd, section::LIST))
        }
        None => {
            Err(missing_cmd_error(section::LIST))
        }
    }
}