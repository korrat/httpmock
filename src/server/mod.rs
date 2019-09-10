extern crate futures;
extern crate hyper;

mod routes;

use self::hyper::header::HeaderValue;
use self::hyper::http::header::HeaderName;
use self::hyper::service::service_fn;
use self::hyper::{HeaderMap, Request, StatusCode};
use futures::future;
use hyper::rt::Future;
use hyper::{Body, Response, Server};
use log::info;
use routes::{HandlerConfig, HttpMockHandlerRequest, HttpMockHandlerResponse};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

#[derive(TypedBuilder, Debug)]
pub struct ServerConfig {
    pub port: u16,
}

pub fn start_server(server_config: ServerConfig) {
    let socket_address = ([127, 0, 0, 1], server_config.port).into();
    let handler_config = Arc::new(
        HandlerConfig::builder()
            .routes(routes::create_routes())
            .build(),
    );

    let server = Server::bind(&socket_address)
        .serve(move || {
            let handler_config = handler_config.clone();
            service_fn(move |native_request: Request<Body>| {
                handle_native_request(&handler_config, native_request)
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    info!("Listening on {}", socket_address);
    hyper::rt::run(server);
}

fn handle_native_request(handler_config: &HandlerConfig, native_request: Request<Body>) -> BoxFut {
    let framework_request = to_framework_request(&native_request);
    let framework_response = handle_framework_request(handler_config, framework_request);
    let native_response = to_native_response(framework_response);

    info!(
        "{} {} {}",
        native_request.method(),
        native_request.uri(),
        native_response.status()
    );

    Box::new(future::ok(native_response))
}

fn to_framework_request(req: &Request<Body>) -> HttpMockHandlerRequest {
    let req_path = req.uri().path().to_string();
    let req_method = req.method().as_str().to_string();
    let req_headers = HashMap::new();
    let _req_body = req.body();

    let handler_request = HttpMockHandlerRequest::builder()
        .method(req_method)
        .path(req_path)
        .headers(req_headers)
        .body(String::new())
        .build();

    handler_request
}

fn to_native_response(handler_response: HttpMockHandlerResponse) -> Response<Body> {
    let mut response = Response::new(Body::from(handler_response.body));
    *response.status_mut() = StatusCode::from_u16(handler_response.status_code)
        .expect("Cannot parse status code from routes");
    *response.headers_mut() = to_headers(&handler_response.headers);
    response
}

fn to_headers(headers: &HashMap<String, String>) -> HeaderMap<HeaderValue> {
    let mut header_map = HeaderMap::with_capacity(headers.capacity());
    for (k, v) in headers {
        let hv = HeaderValue::from_str(v).expect(&format!("Cannot create header value from {}", v));
        let hn = HeaderName::from_str(k).expect(&format!("Cannot create header name from {}", k));
        header_map.insert(hn, hv);
    }
    return header_map;
}

pub fn handle_framework_request(
    handler_config: &HandlerConfig,
    request: HttpMockHandlerRequest,
) -> HttpMockHandlerResponse {
    let handler = handler_config
        .routes
        .iter()
        .find(|&rh| rh.path_regex.is_match(&request.path));

    if let Some(rh) = handler {
        return (rh.handler)(handler_config, request);
    }

    HttpMockHandlerResponse::builder()
        .status_code(404 as u16)
        .headers(HashMap::new())
        .body(String::new())
        .build()
}