use fastly::{
    http::{header, Method, StatusCode},
    Error, Request, Response,
};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version.
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap(),
    );

    // Disallow any mutation HTTP methods.
    match req.get_method() {
        &Method::POST | &Method::PUT | &Method::DELETE | &Method::PATCH => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n"));
        }
        _ => {}
    };

    // Disallow any non-root paths.
    if req.get_path() != "/" {
        return Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n"));
    };

    // Send a boring default response.
    Ok(Response::from_status(StatusCode::OK)
        .with_header(header::CACHE_CONTROL, "public, s-maxage=3600")
        .with_body_text_plain("Hello, Fastly!"))
}
