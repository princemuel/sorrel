mod error;
mod lexer;
mod model;
mod parser;

pub mod prelude;
mod server;

use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use cli::{Args, Subcommand};
pub use error::GlobalError;

use crate::model::{JsonModel, Model};
use crate::parser::Parser;
use crate::server::serve;

pub const DATABASE_JSON: &str = "db.json";
pub const DATABASE_SQLITE: &str = "db.sqlite";

pub fn run(args: &Args) -> Result<(), GlobalError> {
    match &args.subcommand {
        Subcommand::Index { path } => {
            let path = Path::new(path);

            let mut dirs = vec![path.to_path_buf()];
            let mut model = JsonModel::default();

            let add_to_idx = |path: &Path, model: &mut dyn Model| -> Result<(), GlobalError> {
                eprintln!("indexing {path}...", path = path.display());

                let doc = Parser::read_by_ext(path)
                    .map_err(|e| eprintln!("ERROR: {e}"))
                    .ok();
                if let Some(content) = doc {
                    model.add_doc(path, &content)?;
                }

                Ok(())
            };

            while let Some(dir) = dirs.pop() {
                for entry in fs::read_dir(&dir)?.flatten() {
                    let path = entry.path();
                    let r#type = entry.file_type()?;

                    #[expect(clippy::filetype_is_file)]
                    #[expect(clippy::else_if_without_else)]
                    if r#type.is_dir() {
                        dirs.push(path);
                    } else if r#type.is_file() && File::open(&path).is_ok() {
                        add_to_idx(&path, &mut model)?;
                    }
                }
            }

            eprintln!("saving to index '{DATABASE_JSON}'...");

            // TODO: use sqlite3 to store the index
            let writer = BufWriter::new(File::create(DATABASE_JSON)?);
            serde_json::to_writer(writer, &model)?;
        }

        Subcommand::Search { query } => {
            let path = PathBuf::from(DATABASE_JSON);
            println!("Reading the index file...");

            let reader = BufReader::new(File::open(&path)?);
            let model: JsonModel = serde_json::from_reader(reader)?;

            println!("searching for '{query}'...");

            for (path, rank) in model.search(query)?.iter().take(20) {
                println!("{path} {rank}", path = path.display());
            }
        }

        Subcommand::Serve { path, address } => {
            let reader = BufReader::new(File::open(path)?);

            let model: JsonModel = serde_json::from_reader(reader)?;
            serve(address, &model)?;
        }
    }

    Ok(())
}
