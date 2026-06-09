#![expect(clippy::unwrap_used)]

use core::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::path::Path;

use tiny_http::{Header, Method, Request, Response, Server};

use crate::error::Error;
use crate::lexer::Lexer;
use crate::{DocFreq, Model, TermFreqMap};

pub(crate) fn run<P, S>(path: P, address: &S) -> Result<(), Error>
where
    P: AsRef<Path>,
    S: ToSocketAddrs + Display,
{
    let http_server = Server::http(address)?;
    println!("INFO: listening on http://{address}");

    let path = path.as_ref();

    let reader = BufReader::new(File::open(path)?);
    let index = serde_json::from_reader(reader)?;

    for req in http_server.incoming_requests() {
        serve_request(&index, req)?;
    }

    Ok(())
}

fn serve_request(model: &Model, req: Request) -> Result<(), Error> {
    println!("INFO: received request! method: {:?}, url: {:?}", req.method(), req.url());

    match (req.method(), req.url()) {
        (Method::Post, "/api/search") => {
            serve_api_search(model, req)?;
        }
        (Method::Get, "/index.js") => {
            serve_static(req, include_bytes!("index.js"), "text/javascript; charset=utf-8")?;
        }
        (Method::Get, "/" | "/index.html") => {
            serve_static(req, include_bytes!("index.html"), "text/html; charset=utf-8")?;
        }
        _ => serve_404(req)?,
    }

    Ok(())
}

fn search_term<'a>(model: &'a Model, term: &'a str) -> Vec<(&'a Path, f32)> {
    let mut results = Vec::new();
    let mut count = 0;
    for (path, table) in &model.tf {
        let mut rank = 0f32;

        for token in Lexer::new(term) {
            rank += compute_tf(token.lexeme, table)
                * compute_idf(token.lexeme, model.tf.len(), &model.df);
            count += 1;
        }

        results.push((path.as_path(), rank));
    }
    println!("-----------------------------");
    println!("{count}");
    println!("-----------------------------");
    // TODO: cleanup this unwrap later
    results.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    results
}

fn serve_api_search(model: &Model, mut req: Request) -> Result<(), Error> {
    let mut buf = Vec::new();
    req.as_reader().read_to_end(&mut buf)?;
    let body = str::from_utf8(&buf).map_err(|e| Error::Other(Box::new(e)))?;

    let results = search_term(model, body);

    let json = serde_json::to_string(&results.iter().take(20).collect::<Vec<_>>())?;
    let content_type = Header::from_bytes("Content-Type", "application/json").unwrap();

    Ok(req.respond(Response::from_string(json).with_header(content_type))?)
}

fn serve_static(req: Request, bytes: &[u8], content_type: &str) -> Result<(), Error> {
    // TODO: cleanup this unwrap later
    let content_type = Header::from_bytes("Content-Type", content_type).unwrap();
    let response = Response::from_data(bytes).with_header(content_type);
    req.respond(response)?;
    Ok(())
}

fn serve_404(req: Request) -> Result<(), Error> {
    Ok(req.respond(Response::from_string("404: Not Found").with_status_code(404u16))?)
}

fn _serve_500(req: Request) -> Result<(), Error> {
    Ok(req.respond(Response::from_string("500: Internal Server Error").with_status_code(500u16))?)
}

#[expect(clippy::cast_precision_loss, clippy::as_conversions)]
fn compute_tf(t: &str, d: &TermFreqMap) -> f32 {
    let a = d.get(t).copied().unwrap_or(0) as f32;
    let b = d.values().map(|freq| *freq as f32).sum::<f32>();
    a / b
}

#[expect(clippy::cast_precision_loss, clippy::as_conversions)]
fn compute_idf(t: &str, n: usize, d: &DocFreq) -> f32 {
    let n = n as f32;
    let m = d.get(t).copied().unwrap_or(1) as f32;
    (n / m).log10()
}
