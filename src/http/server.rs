use std::{error::Error, sync::Arc};

use tokio::net::TcpListener;

use super::handle::Handler;

pub struct Server {
    handle: Arc<Handler>
}

impl Server {
    pub fn new(handler: Handler) -> Self {
        Self {
            handle: Arc::new(handler)
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind("127.0.0.1:3000").await?;
        loop {
            let (stream, _addr) = listener.accept().await?;
            let handler = Arc::clone(&self.handle);

            tokio::spawn(async move {
                handler.process_client(stream).await.unwrap();
            });
        }
    }
}