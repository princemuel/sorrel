use lexopt::Parser;

use crate::error::Error;
#[expect(dead_code)]
pub(crate) struct Args {
    pub thing: String,
    pub number: u32,
    pub shout: bool,
}

impl Args {
    pub(crate) fn _parse() -> Result<Self, Error> {
        use lexopt::prelude::*;

        let mut thing = None;
        let mut number = 1;
        let mut shout = false;
        let mut parser = Parser::from_env();

        while let Some(arg) = parser.next()? {
            match arg {
                Short('n') | Long("number") => {
                    number = parser.value()?.parse()?;
                }
                Long("shout") => {
                    shout = true;
                }
                Value(val) if thing.is_none() => {
                    thing = Some(val.string()?);
                }
                Short('h') | Long("help") => {
                    println!("Usage: hello [-n|--number=NUM] [--shout] THING");
                    std::process::exit(0);
                }
                _ => return Err(Error::Cli(arg.unexpected())),
            }
        }

        Ok(Args {
            thing: thing.ok_or_else(|| Error::Unexpected("missing argument THING".to_owned()))?,
            number,
            shout,
        })
    }
}
