use std::io;

use ntex::{
    channel::mpsc,
    http::{header, Method, StatusCode},
    util::Bytes,
    web::{self, error, Error, HttpRequest, HttpResponse}
};
use ntex_files as fs;
use ntex_session::Session;
use web::types::Path;

pub(super) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service((
        ping,
        // web::service("/ping", web::get().to(ping)),
        // register favicon
        favicon,
        // register simple route, handle all methods
        welcome,
        // register match_all_paths method
        match_all_paths,
        // with path parameters
        web::resource("/user/{name}").route(web::get().to(with_param)),
        // async response body
        web::resource("/async-body/{name}").route(web::get().to(response_body)),
        web::resource("/test").to(|req: HttpRequest| async move {
            match *req.method() {
                Method::GET => HttpResponse::Ok(),
                Method::POST => HttpResponse::MethodNotAllowed(),
                _ => HttpResponse::NotFound()
            }
        }),
        web::resource("/error").to(|| async {
            error::InternalError::new(
                io::Error::new(io::ErrorKind::Other, "test"),
                StatusCode::INTERNAL_SERVER_ERROR
            )
        }),
        // static files
        fs::Files::new("/static", "public").show_files_listing(),
        // redirect
        web::resource("/").route(web::get().to(
            |req: HttpRequest| async move {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/welcome.html")
                    .finish()
            }
        ))
    ));
}

/// response body
async fn response_body(path: Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(
    req: HttpRequest,
    path: Path<(String,)>
) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}

/// Handler to match all paths starting with /files
#[web::get("/files/{all}*")]
async fn match_all_paths(path: Path<String>) -> HttpResponse {
    println!("path: {:?}", path);
    HttpResponse::Ok().content_type("text/plain").body("it's matching !")
}

/// favicon handler
#[web::get("/favicon")]
async fn favicon() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("../../../../public/favicon.ico")?)
}
/// favicon handler
#[web::get("/ping")]
async fn ping() -> &'static str {
    "PONG"
}

/// simple index handler

#[web::get("/welcome")]
async fn welcome(
    session: Session,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../../public/welcome.html")))
}
