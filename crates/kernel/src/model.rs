use core::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::lexer::Lexer;
use crate::prelude::GlobalError;

pub(crate) trait Model {
    fn search(&self, query: &str) -> Result<Vec<(PathBuf, PathBuf, f32)>, GlobalError>;
    fn add_document(&mut self, path: &Path, content: &str) -> Result<(), GlobalError>;
}

pub(crate) struct SqliteModel;

impl SqliteModel {}
impl Model for SqliteModel {
    fn search(&self, _query: &str) -> Result<Vec<(PathBuf, PathBuf, f32)>, GlobalError> { todo!() }

    fn add_document(&mut self, _path: &Path, _content: &str) -> Result<(), GlobalError> { todo!() }
}

type Docs = HashMap<PathBuf, Doc>;
type DocFreq = HashMap<String, usize>;
type TermFreq = HashMap<String, usize>;

#[derive(Debug, Default, Deserialize, Serialize)]
struct Doc {
    file: PathBuf,
    tf: TermFreq,
    count: usize,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct JsonModel {
    docs: Docs,
    df: DocFreq,
}

impl Model for JsonModel {
    fn search(&self, query: &str) -> Result<Vec<(PathBuf, PathBuf, f32)>, GlobalError> {
        let mut results = vec![];

        let tokens: Vec<_> = Lexer::new(query).collect();

        for (path, doc) in &self.docs {
            let mut rank = 0f32;

            for term in &tokens {
                rank += compute_tf(term, doc) * compute_idf(term, self.docs.len(), &self.df);
            }

            results.push((path.to_owned(), path.canonicalize()?, rank));
        }

        results.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        Ok(results)
    }

    fn add_document(&mut self, path: &Path, content: &str) -> Result<(), GlobalError> {
        let mut tf = TermFreq::new();
        let mut count = 0;
        let tokens: Vec<_> = Lexer::new(content).collect();

        for term in tokens {
            tf.entry(term.to_string()).and_modify(|freq| *freq += 1).or_insert(1);
            count += 1;
        }

        // TODO: why not insert to the term freq and doc freq at the same time?
        for t in tf.keys() {
            self.df.entry(t.to_owned()).and_modify(|freq| *freq += 1).or_insert(1);
        }

        self.docs.insert(path.to_path_buf(), Doc { tf, count, file: path.canonicalize()? });

        Ok(())
    }
}

#[expect(clippy::cast_precision_loss, clippy::as_conversions)]
fn compute_tf(t: &str, d: &Doc) -> f32 {
    let n = d.count as f32;
    let m = d.tf.get(t).copied().unwrap_or(0) as f32;
    m / n
}

#[expect(clippy::cast_precision_loss, clippy::as_conversions)]
fn compute_idf(t: &str, n: usize, df: &DocFreq) -> f32 {
    let n = n as f32;
    let m = df.get(t).copied().unwrap_or(1) as f32;
    (n / m).log10()
}
