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
use hyper::{client::HttpConnector, Body, Client};
use hyper_native_tls::NativeTlsClient;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tokio::spawn(server());

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let app = Router::new()
        .route(NetworkProxyHandler::PATH, any(handler))
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
#[typed_path("/:network")]
struct NetworkProxyHandler {
    network: String,
}

async fn handler(
    params: NetworkProxyHandler,
    State(client): State<Client<HttpsConnector<HttpConnector>>>,
    mut req: Request<Body>,
) -> Response<Body> {
    // let mut body = vec![];
    // return Response::new(Body::from(params.network));
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    println!("path_query: {}", path_query);
    let uri = format!("https://rpc-juno.pupmos.network{}", path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}

async fn server() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
