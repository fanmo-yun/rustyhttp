use http::{handle::Handler, server::Server};

mod http;

#[tokio::main]
async fn main() {
    let webserver = Server::new(
        Handler::new()
    );
    webserver.run().await.unwrap();
}
