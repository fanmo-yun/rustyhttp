use http::{handle::Handler, server::Server};

pub mod http;
pub mod utils;

#[tokio::main]
async fn main() {
    let webserver = Server::new(Handler::default());
    webserver.run().await.unwrap();
}
