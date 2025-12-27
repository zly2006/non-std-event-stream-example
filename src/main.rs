use async_stream::__private::AsyncStream;
use axum::{body::Body, http::{HeaderMap, HeaderValue, StatusCode}, response::{IntoResponse, Response}, routing::post, Error, Router};
use std::time::Duration;

async fn handler(
) -> Result<Response, (StatusCode, ())> {
    let output_stream: AsyncStream<Result<bytes::Bytes, Error>, _> = async_stream::stream! {
        yield Ok(bytes::Bytes::from_static(b"event:no-space-here\ndata:data: message1\n\n"));
        tokio::time::sleep(Duration::from_secs(3)).await;
        yield Ok(bytes::Bytes::from_static(b"event:final-event\ndata:message2\n\n"));
    };

    let body = Body::from_stream(output_stream);
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_static("text/event-stream"),
    );
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    headers.insert("connection", HeaderValue::from_static("keep-alive"));
    headers.insert("transfer-encoding", HeaderValue::from_static("chunked"));
    headers.insert("set-cookie", HeaderValue::from_static("I18nextLngHiagent=dev; Path=/; Expires=Sun, 27 Dec 2026 16:40:30 GMT; HttpOnly; SameSite=Strict"));
    headers.insert("Vary", HeaderValue::from_static("Accept-Encoding"));

    Ok((headers, body).into_response())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/sse", post(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
