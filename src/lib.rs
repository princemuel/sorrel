mod cli;
mod error;
mod lexer;
mod model;
mod parser;
mod server;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub use cli::{Action, Args, Subcommand};
pub use error::Error;
use serde::{Deserialize, Serialize};
use xml::EventReader;
use xml::common::{Position as _, TextPosition};
use xml::reader::XmlEvent;

use crate::lexer::Lexer;

type DocFreq = HashMap<String, usize>;
type TermFreqMap = HashMap<String, usize>;
type TermFreqPerDoc = HashMap<PathBuf, (usize, TermFreqMap)>;

pub const DATABASE: &str = "db.local.json";

#[derive(Debug, Default, Deserialize, Serialize)]
struct Model {
    pub tf: TermFreqPerDoc,
    pub df: DocFreq,
}

pub fn run(args: &Args) -> Result<(), Error> {
    match &args.subcommand {
        Subcommand::Serve { path, address } => server::run(path, address)?,
        Subcommand::Index { path } => index(&*path.as_os_str().to_string_lossy())?,
        Subcommand::Search { query } => search(query)?,
    }

    Ok(())
}

pub fn search(query: &str) -> Result<(), Error> {
    println!("searching for '{query}'...");

    let path = PathBuf::from(DATABASE);
    println!("Reading the index file...");

    let reader = BufReader::new(File::open(&path)?);
    let index: TermFreqPerDoc = serde_json::from_reader(reader)?;
    println!(
        "The index '{path}' contains {count} files",
        path = &path.display(),
        count = index.len()
    );
    Ok(())
}

pub fn index<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let mut model = Model::default();

    let mut dirs = vec![path.to_path_buf()];

    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir)?.flatten() {
            let path = entry.path();

            #[expect(clippy::else_if_without_else)]
            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() && File::open(&path).is_ok() {
                add_to_index(&path, &mut model)?;
            }
        }
    }

    eprintln!("saving to index '{DATABASE}'...");

    // TODO: use sqlite3 to store the index
    let writer = BufWriter::new(File::create(DATABASE)?);
    serde_json::to_writer(writer, &model)?;

    Ok(())
}

fn read_xml<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = EventReader::new(BufReader::new(file));

    let mut buffer = String::new();
    for event in reader {
        let event = event.map_err(|err| {
            let TextPosition { row, column } = err.position();
            eprintln!("{path}:{row}:{column}: ERROR: {err}", path = path.display());
            io::Error::other(err)
        })?;

        if let XmlEvent::Characters(ref text) = event {
            buffer.push_str(text);
            buffer.push(' ');
        }
    }

    Ok(buffer)
}

fn add_to_index(path: &Path, model: &mut Model) -> io::Result<()> {
    eprintln!("indexing {path}...", path = path.display());

    let content = match path.extension().and_then(OsStr::to_str) {
        Some("xml" | "xhtml") => read_xml(path)?,
        Some("txt" | "md") => fs::read_to_string(path)?,
        _ => return Ok(()), // skip unsupported files
    };

    let mut tf = TermFreqMap::new();

    for token in Lexer::new(&content) {
        tf.entry(token.lexeme.to_owned()).and_modify(|e| *e += 1).or_insert(1);
    }

    for t in tf.keys() {
        model.df.entry(t.to_owned()).and_modify(|entry| *entry += 1).or_insert(1);
    }

    model.tf.insert(path.to_path_buf(), tf);

    Ok(())
}
