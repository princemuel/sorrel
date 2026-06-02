#![expect(clippy::unwrap_used)]

use core::net::SocketAddr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tiny_http::{Header, Method, Request, Response, Server};

use crate::TermFreqIdx;
use crate::error::Error;
use crate::lexer::Lexer;

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

fn serve_request(_index: &TermFreqIdx, mut req: Request) -> Result<(), Error> {
    println!("INFO: received request! method: {:?}, url: {:?}", req.method(), req.url());

    match (req.method(), req.url()) {
        (Method::Post, "/api/search") => {
            let mut buf = Vec::new();
            req.as_reader().read_to_end(&mut buf)?;
            let body = str::from_utf8(&buf).map_err(|e| Error::Other(Box::new(e)))?;

            for token in Lexer::new(body) {
                let term: String = token.into();
                println!("{term}");
            }

            req.respond(Response::from_string("ok"))?;
        }
        (Method::Get, "/index.js") => {
            serve_static_file(req, "public/index.js", "text/javascript; charset=utf-8")?;
        }
        (Method::Get, "/") => {
            serve_static_file(req, "public/index.html", "text/html; charset=utf-8")?;
        }
        _ => serve_not_found(req)?,
    }

    Ok(())
}

fn serve_static_file(
    req: Request,
    path: impl AsRef<Path>,
    content_type: &str,
) -> Result<(), Error> {
    let file = File::open(path)?;
    let content_type = Header::from_bytes("Content-Type", content_type).unwrap();
    let response = Response::from_file(file).with_header(content_type);
    req.respond(response)?;
    Ok(())
}

fn serve_not_found(req: Request) -> Result<(), Error> {
    req.respond(Response::from_string("404: Not Found").with_status_code(404u16))?;

    Ok(())
}
