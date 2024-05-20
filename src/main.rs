use http::server;

mod http;

#[tokio::main]
async fn main() {
    server::init_webserver().await;
}
