use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ffi::OsString;

use lexopt::{Error, Parser};

pub enum Subcommand {
    Serve { path: OsString, address: SocketAddr },
    Index { path: OsString },
    Search { query: String },
}

pub struct Args {
    pub subcommand: Subcommand,
}

pub enum Action {
    Help,
    Run(Args),
}

const DEFAULT_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

impl Args {
    pub fn parse() -> Result<Action, Error> {
        use lexopt::prelude::*;

        let mut parser = Parser::from_env();

        let subcommand = match parser.next()? {
            Some(Value(v)) => v.string()?,
            Some(Short('h') | Long("help")) => return Ok(Action::Help),
            Some(arg) => return Err(arg.unexpected()),
            None => return Err(Error::Custom("no subcommand provided".into())),
        };

        let subcommand = match subcommand.as_str() {
            "serve" => {
                let mut path = None;
                let mut address = None;

                while let Some(arg) = parser.next()? {
                    match arg {
                        Short('h') | Long("help") => return Ok(Action::Help),
                        Value(v) if path.is_none() => {
                            path = Some(v);
                        }
                        Value(v) if address.is_none() => {
                            address = Some(v.string()?.parse().unwrap_or(DEFAULT_ADDRESS));
                        }
                        arg => return Err(arg.unexpected()),
                    }
                }

                let path = path.ok_or_else(|| {
                    Error::Custom("no directory provided for the `serve` command".into())
                })?;

                Subcommand::Serve { path, address: address.unwrap_or(DEFAULT_ADDRESS) }
            }

            "index" => {
                let mut path = None;

                while let Some(arg) = parser.next()? {
                    match arg {
                        Short('h') | Long("help") => return Ok(Action::Help),
                        Value(v) if path.is_none() => {
                            path = Some(v);
                        }
                        arg => return Err(arg.unexpected()),
                    }
                }

                let path = path.ok_or_else(|| {
                    Error::Custom("no path provided for the `index` command".into())
                })?;

                Subcommand::Index { path }
            }

            "search" => {
                let mut query = None;

                while let Some(arg) = parser.next()? {
                    match arg {
                        Short('h') | Long("help") => return Ok(Action::Help),
                        Value(v) if query.is_none() => {
                            query = Some(v.string()?);
                        }
                        arg => return Err(arg.unexpected()),
                    }
                }

                let query = query.ok_or_else(|| {
                    Error::Custom("no query provided for the `search` command".into())
                })?;

                Subcommand::Search { query }
            }
            "help" => return Ok(Action::Help),
            command => {
                return Err(Error::Custom(format!("unknown subcommand `{command}`").into()));
            }
        };

        Ok(Action::Run(Args { subcommand }))
    }

    pub fn usage() {
        eprint!(
            "\
A Local-First Search Engine Indexer

Usage:
    sorrel[EXE] <COMMAND> [OPTIONS]

Commands:
    serve <folder> [address]        Start a local HTTP server with a Web Interface
                                    address defaults to 127.0.0.1:8080
    index <folder>                  index the folder and save the index to the index file
    search <index-file>             Search the index file for the given query
    help  Print this message or the help of the given subcommand(s)

Options:
    -h, --help    Print this help and exit
    "
        );
    }
}
