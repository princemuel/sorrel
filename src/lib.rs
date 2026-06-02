mod cli;
mod error;
mod lexer;
mod parser;
mod server;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write as _};
use std::path::{Path, PathBuf};

pub use cli::{Action, Args, Subcommand};
pub use error::Error;
use xml::EventReader;
use xml::reader::XmlEvent;

use crate::lexer::Lexer;

type TermFreqMap = HashMap<String, usize>;
type TermFreqIdx = HashMap<PathBuf, TermFreqMap>;

pub fn run(args: &Args) -> Result<(), Error> {
    match &args.subcommand {
        Subcommand::Serve { path, address } => server::run(path, address)?,
        Subcommand::Index { path } => index(&path.as_os_str().to_string_lossy())?,
        Subcommand::Search { query } => search(query)?,
    }

    Ok(())
}

pub fn search(query: &str) -> Result<(), Error> {
    println!("searching for '{query}'...");

    let path = PathBuf::from(query);
    println!("Reading the index file...");

    let reader = BufReader::new(File::open(&path)?);
    let index: TermFreqIdx = serde_json::from_reader(reader)?;
    println!(
        "The index '{path}' contains {count} files",
        path = &path.display(),
        count = index.len()
    );
    Ok(())
}

pub fn index(path: &str) -> Result<(), Error> {
    let path = PathBuf::from(path);
    let files = read_dirs(&path)?;

    let mut docs = TermFreqIdx::new();
    let mut stderr = BufWriter::new(io::stderr().lock());

    'outer: for path in files {
        writeln!(stderr, "indexing {path}...", path = path.display())?;

        let content = match path.extension().and_then(OsStr::to_str) {
            Some("xml" | "xhtml") => read_xml(&path)?,
            Some("txt" | "md") => fs::read_to_string(&path)?,
            _ => continue 'outer, // skip unsupported files
        };

        let mut tf = TermFreqMap::new();

        for token in Lexer::new(&content) {
            let term = token.into();
            tf.entry(term).and_modify(|entry| *entry += 1).or_insert(1);
        }

        // let mut stats: Vec<_> = tf.iter().collect();
        // let n = 100.min(tf.len());
        // stats.select_nth_unstable_by(n.saturating_sub(1), |(_, a), (_, b)| b.cmp(a));
        // stats.get_mut(..n).unwrap_or_default().sort_by(|(_, a), (_, b)| b.cmp(a));

        docs.insert(path, tf);
    }

    let path = "db.local.json";
    writeln!(stderr, "saving to {path}...")?;

    stderr.flush()?;

    let writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer(writer, &docs)?;

    Ok(())
}

fn read_xml<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = BufReader::new(File::open(path)?);
    let parser = EventReader::new(file);

    let mut buffer = String::new();
    for event in parser.into_iter().flatten() {
        if let XmlEvent::Characters(ref text) = event {
            buffer.push_str(text);
            buffer.push(' ');
        }
    }

    Ok(buffer)
}

fn read_dirs<P>(path: P) -> io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut files = Vec::new();
    let mut dirs = vec![path.as_ref().to_path_buf()];

    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            #[expect(clippy::else_if_without_else)]
            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}
