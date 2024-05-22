use std::error::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

#[derive(Debug, Clone, Copy)]
pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }

    pub async fn process_client(self, mut client: TcpStream) -> Result<(), Box<dyn Error>> {
        let (mut reader, mut writer) = client.split();
        let mut buf = [0u8; 1024];
        let n = reader.read(&mut buf).await?;
        println!("{}", String::from_utf8_lossy(&buf[..n]));
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello world";
        writer.write_all(response.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
}