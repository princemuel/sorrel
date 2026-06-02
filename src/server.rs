#![expect(clippy::unwrap_used)]

use core::net::SocketAddr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tiny_http::{Header, Method, Request, Response, Server};

use crate::error::Error;
use crate::lexer::Lexer;
use crate::{TermFreqIdx, TermFreqMap};

pub(crate) fn run<P>(path: P, address: &SocketAddr) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let http_server = Server::http(address)?;

    println!("INFO: listening on http://{address}");

    let path = path.as_ref();

    let reader = BufReader::new(File::open(path)?);
    let index = serde_json::from_reader(reader)?;

    for req in http_server.incoming_requests() {
        serve_request(&index, req)?;
    }

    todo!()
}

fn serve_request(index: &TermFreqIdx, mut req: Request) -> Result<(), Error> {
    println!("INFO: received request! method: {:?}, url: {:?}", req.method(), req.url());

    match (req.method(), req.url()) {
        (Method::Post, "/api/search") => {
            let mut buf = Vec::new();
            req.as_reader().read_to_end(&mut buf)?;
            let body = str::from_utf8(&buf).map_err(|e| Error::Other(Box::new(e)))?;

            let mut results: Vec<(&Path, f32)> = Vec::new();
            for (path, table) in index {
                let mut rank = 0f32;

                for token in Lexer::new(body) {
                    let term: String = token.into();
                    rank += tf(&term, table) * idf(&term, index);
                }

                results.push((path, rank));
            }
            // TODO: cleanup this unwrap later
            results.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

            for (path, rank) in results.iter().take(10) {
                println!("{path} => {rank}", path = path.display());
            }

            req.respond(Response::from_string("ok"))?;
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
fn tf(t: &str, d: &TermFreqMap) -> f32 {
    *d.get(t).unwrap_or(&0) as f32 / d.values().map(|freq| *freq as f32).sum::<f32>()
}

#[expect(clippy::cast_precision_loss, clippy::as_conversions)]
fn idf(t: &str, d: &TermFreqIdx) -> f32 {
    let n = d.len() as f32;
    let m = d.values().filter(|tf| tf.contains_key(t)).count().max(1) as f32;
    (n / m).log10()
}
