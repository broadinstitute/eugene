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
    Symbol { species: Species, symbol: Symbol }
}

pub(crate) enum UtilConfig {
    SymbolToGeneId { species: Species, symbol: Symbol }
}

mod section {
    pub(crate) const XREFS: &str = "xrefs";
    pub(crate) const LOOKUP: &str = "lookup";
    pub(crate) const UTIL: &str = "util";
    pub(crate) const LIST: &[&str] = &[XREFS, LOOKUP, UTIL];
}

mod cmd {
    pub(crate) const SYMBOL: &str = "symbol";
    pub(crate) const SYMBOL_TO_GENE_ID: &str = "symbol-to-gene-id";
    pub(crate) const XREFS_CMDS: &[&str] = &[SYMBOL];
    pub(crate) const LOOKUP_CMDS: &[&str] = &[SYMBOL];
    pub(crate) const UTIL_CMDS: &[&str] = &[SYMBOL_TO_GENE_ID];
}

mod arg {
    use clap::Arg;

    pub(crate) const SPECIES: &str = "species";
    pub(crate) const SYMBOL: &str = "symbol";

    pub(crate) fn species() -> Arg {
        Arg::new(SPECIES).short('p').long(SPECIES).required(false)
    }

    pub(crate) fn symbol() -> Arg { Arg::new(SYMBOL).short('s').long(SYMBOL) }
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