use axum::body::Body;
use axum::{http::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn log_requests(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;

    println!(
        "{} {} -> {} ({}ms)",
        method,
        uri,
        response.status(),
        start.elapsed().as_millis()
    );

    response
}
