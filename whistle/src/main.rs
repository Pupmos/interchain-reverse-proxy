//!
//! Run with
//!
//! ```not_rust
//! cargo run -p whistle
//! ```

use axum::{
    extract::State,
    http::{uri::Uri, Request, Response},
    routing::{any, get},
    Router,
};
use axum_extra::routing::{
    RouterExt, // for `Router::typed_get`
    TypedPath,
};
use hyper::{client::HttpConnector, header::ToStrError, Body, Client};
use hyper_native_tls::NativeTlsClient;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let app = Router::new()
        .route(NetworkProxyHandler::PATH, any(handler))
        .route(BareNetworkProxyHandler::PATH, any(handler))
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("reverse proxy listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// A type-safe path
#[derive(TypedPath, Deserialize)]
#[typed_path("/:network/*path")]
struct NetworkProxyHandler {
    network: String,
    path: String,
}

// A type-safe path
#[derive(TypedPath, Deserialize)]
#[typed_path("/:network")]
struct BareNetworkProxyHandler {
    network: String,
}

async fn handler(
    params: BareNetworkProxyHandler,
    State(client): State<Client<HttpsConnector<HttpConnector>>>,
    mut req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    println!("path_query: {}", path_query);
    let path_query = path_query.replace(format!("/{}", params.network).as_str(), "");
    let uri = format!(
        "https://rpc-{}.pupmos.network{}",
        params.network, path_query
    );
    println!("uri: {}", uri);
    let fresh_req = Request::builder()
        .method(req.method().clone())
        .uri(uri)
        .body(req.into_body())
        .unwrap();
    client.request(fresh_req).await.unwrap()
}
