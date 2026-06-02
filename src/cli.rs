use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use lexopt::{Error, Parser};

// The subcommand enum — each variant owns its arguments
pub(crate) enum Subcommand {
    Serve { path: PathBuf, address: SocketAddr },
}

pub(crate) struct Args {
    pub subcommand: Subcommand,
}

impl Args {
    pub(crate) fn parse() -> Result<Self, Error> {
        use lexopt::prelude::*;

        let parser = Parser::from_env();

        // lexopt doesn't expose argv[0] directly, but you can get it if needed:
        let program = parser.bin_name().unwrap_or("sorrel");

        // TODO: cloned this due to the immutable borrow earlier
        let mut parser = parser.clone();

        // First positional value is the subcommand
        let subcommand = match parser.next()? {
            Some(Value(v)) => v.string()?,
            Some(arg) => return Err(arg.unexpected()),
            None => {
                eprintln!("ERROR: no subcommand is provided");
                eprintln!("USAGE: {program} <subcommand> [args]");
                return Err(Error::Custom("no subcommand provided".into()));
            }
        };

        let subcommand = match subcommand.as_str() {
            "serve" => {
                // --- positional: path ---
                let path = match parser.next()? {
                    Some(Value(v)) => PathBuf::from(v.string()?),
                    Some(arg) => return Err(arg.unexpected()),
                    None => {
                        eprintln!("ERROR: no directory is provided for `serve` subcommand");
                        return Err(Error::Custom("missing path for serve".into()));
                    }
                };

                // --- optional positional: address ---
                let address = match parser.next()? {
                    Some(Value(v)) => v.string()?,
                    Some(arg) => return Err(arg.unexpected()),
                    None => "127.0.0.1:8080".to_owned(),
                };

                let address = address
                    .parse()
                    .unwrap_or_else(|_| SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080));

                Subcommand::Serve { path, address }
            }

            other => {
                eprintln!("ERROR: unknown subcommand `{other}`");
                eprintln!("USAGE: {program} <subcommand> [args]");
                return Err(Error::Custom(format!("unknown subcommand `{other}`").into()));
            }
        };

        Ok(Args { subcommand })
    }
}
