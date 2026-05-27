mod cli;
mod error;
mod lexer;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

pub use error::Error;
use xml::EventReader;
use xml::reader::XmlEvent;

use crate::lexer::{Lexer, Token};

type TermFreqMap = HashMap<String, usize>;

pub fn run() -> Result<(), Error> {
    let path = "local/docs.gl";
    let files = read_dirs(path)?;

    let mut docs = HashMap::new();

    for path in files {
        let content = match path.extension().and_then(|e| e.to_str()) {
            Some("xml" | "xhtml") => read_xml(&path)?,
            Some("txt" | "md") => fs::read_to_string(&path)?,
            _ => continue, // skip unsupported files
        };

        let mut tf = TermFreqMap::new();

        for token in Lexer::new(&content) {
            let term = match token {
                Token::Str(t) => t.to_ascii_uppercase(),
                Token::Num(n) => n.to_string(),
            };

            tf.entry(term).and_modify(|f| *f += 1).or_insert(1);
        }

        let mut stats: Vec<_> = tf.iter().collect();
        let n = 100.min(tf.len());
        stats.select_nth_unstable_by(n.saturating_sub(1), |(_, a), (_, b)| b.cmp(a));
        stats.get_mut(..n).unwrap_or_default().sort_by(|(_, a), (_, b)| b.cmp(a));

        println!("{}", path.display());
        for (term, freq) in stats.get(..n).unwrap_or_default() {
            println!("    {term} => {freq}");
        }

        docs.insert(path, tf);
    }

    Ok(())
}

fn read_xml<P: AsRef<Path>>(path: P) -> io::Result<String> {
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
            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }

    Ok(files)
}
