use core::fmt::Display;
use std::io;
use std::net::ToSocketAddrs;

use tiny_http::{Header, Method, Request, Response, Server};

use crate::error::GlobalError;
use crate::model::Model;

pub(crate) fn serve<S>(address: &S, model: &impl Model) -> Result<(), GlobalError>
where
    S: ToSocketAddrs + Display,
{
    let http_server = Server::http(address)?;
    println!("INFO: listening on http://{address}");

    for req in http_server.incoming_requests() {
        // * don't stop on errors, keep serving
        serve_request(req, model).ok();
    }

    Ok(())
}

fn serve_request(req: Request, model: &impl Model) -> io::Result<()> {
    println!("INFO: received request! method: {:?}, url: {:?}", req.method(), req.url());

    match (req.method(), req.url()) {
        (Method::Post, "/api/search") => {
            serve_api_search(req, model)?;
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

// TODO: the errors of serve_api_search should probably return JSON
fn serve_api_search(mut req: Request, model: &impl Model) -> io::Result<()> {
    let mut buf = Vec::new();
    req.as_reader().read_to_end(&mut buf)?;

    let query = match str::from_utf8(&buf) {
        Ok(body) => body.trim(),
        Err(e) => {
            eprintln!("ERROR: could not interpret body as UTF-8 string: {e}");
            return serve_400(req, "Body must be a valid UTF-8 string");
        }
    };

    let results = model.search(query);

    let json = match serde_json::to_string(&results.iter().take(20).collect::<Vec<_>>()) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("ERROR: could not convert to json: {e}");
            return serve_500(req);
        }
    };

    // TODO: cleanup this unwrap later
    #[expect(clippy::unwrap_used)]
    let content_type = Header::from_bytes("Content-Type", "application/json").unwrap();
    req.respond(Response::from_string(json).with_header(content_type))
}

fn serve_static(req: Request, bytes: &[u8], content_type: &str) -> io::Result<()> {
    // TODO: cleanup this unwrap later
    #[expect(clippy::unwrap_used)]
    let content_type = Header::from_bytes("Content-Type", content_type).unwrap();
    let response = Response::from_data(bytes).with_header(content_type);
    req.respond(response)
}

fn serve_400(req: Request, msg: &str) -> io::Result<()> {
    req.respond(Response::from_string(format!("400: Bad Request '{msg}'")).with_status_code(400u16))
}

fn serve_404(req: Request) -> io::Result<()> {
    req.respond(Response::from_string("404: Not Found").with_status_code(404u16))
}

fn serve_500(req: Request) -> io::Result<()> {
    req.respond(Response::from_string("500: Internal Server Error").with_status_code(500u16))
}
