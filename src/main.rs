use config::conf_api::read_config;
use http::{handle::Handler, server::Server};

pub mod config;
pub mod http;
pub mod utils;

#[tokio::main]
async fn main() {
    let _c = read_config().await;
    let webserver = Server::new(Handler::default());
    webserver.run().await.unwrap();
}
