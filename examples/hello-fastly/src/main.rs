use fastly::{
    http::{header, Method, StatusCode},
    Request, Response,
};

fn main() {
    // Get the current Fastly request and pass it to our request handler.
    handle(&Request::from_client()).send_to_client();
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn handle(req: &Request) -> Response {
    // Disallow any mutation HTTP methods.
    match req.get_method() {
        &Method::POST | &Method::PUT | &Method::DELETE | &Method::PATCH => {
            return Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n");
        }
        _ => {}
    };

    // Disallow any non-root paths.
    if req.get_path() != "/" {
        return Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n");
    };

    // Send a boring default response.
    Response::from_status(StatusCode::OK)
        .with_header(header::CACHE_CONTROL, "public, s-maxage=3600")
        .with_body_text_plain("Hello, Fastly!")
}

#[cfg(test)]
mod tests {
    use super::handle;
    use fastly::http::Request;

    #[test]
    fn test_main_refuses_post() {
        let req = Request::post("http://example.com/");
        let resp = handle(&req);
        assert_eq!(resp.get_status(), 405);
    }

    #[test]
    fn test_main_refuses_put() {
        let req = Request::put("http://example.com/");
        let resp = handle(&req);
        assert_eq!(resp.get_status(), 405);
    }

    #[test]
    fn test_main_refuses_delete() {
        let req = Request::delete("http://example.com/");
        let resp = handle(&req);
        assert_eq!(resp.get_status(), 405);
    }

    #[test]
    fn test_main_refuses_patch() {
        let req = Request::patch("http://example.com/");
        let resp = handle(&req);
        assert_eq!(resp.get_status(), 405);
    }

    #[test]
    fn test_main_refuses_non_root_path() {
        let req = Request::get("http://example.com/foo");
        let resp = handle(&req);
        assert_eq!(resp.get_status(), 404);
    }

    #[test]
    fn test_main_hello_world_happy_path() {
        let req = Request::get("http://example.com/");
        let resp = handle(&req);
        assert_eq!(resp.get_status(), 200);
        assert_eq!(resp.into_body_str(), "Hello, Fastly!");
    }
}
