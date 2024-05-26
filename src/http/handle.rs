use std::{error::Error, path::Path};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

use crate::utils::{file_type::FileType, file_utils::Utils};

#[derive(Debug)]
pub struct Handler {
    utils: Utils,
    cap_size: usize
}

impl Default for Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler {
    pub fn new() -> Self {
        Self {
            utils: Utils,
            cap_size: 4096
        }
    }

    async fn read_request<T: AsyncReadExt + Unpin>(
        &self,
        mut reader: BufReader<T>,
    ) -> tokio::io::Result<String> {
        let mut http_request = Vec::new();
        let n = reader.read_until(b'\n', &mut http_request).await?;
        let request_str = String::from_utf8_lossy(&http_request[..n])
            .trim_end()
            .to_string();
        Ok(request_str)
    }

    async fn write_response<T: AsyncWriteExt + Unpin>(
        &self,
        mut writer: BufWriter<T>,
        context: String,
    ) -> tokio::io::Result<()> {
        writer.write_all(context.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }

    async fn process_request_path<T: AsyncReadExt + Unpin, U: AsyncWriteExt + Unpin>(
        &self,
        request: BufReader<T>,
        response: BufWriter<U>,
    ) -> tokio::io::Result<()> {
        let req = self.read_request(request).await?;
        let request_parts: Vec<&str> = req.split_whitespace().collect();
        let method = request_parts[0];
        let path = request_parts[1];

        let res = self.read_file_from_path(method, path).await;
        self.write_response(response, res).await?;
        Ok(())
    }

    // TODO
    async fn read_file_from_path(&self, method: &str, web_path: &str) -> String {
        match (method, web_path) {
            ("GET", path) => {
                return format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    path.len(),
                    path
                );
            },
            _ => {
                return String::from("HTTP/1.1 405 Method Not Allowed\r\n\r\nMethod Not Allowed");
            }
        }
    }

    pub async fn process_client(&self, mut client: TcpStream) -> Result<(), Box<dyn Error>> {
        let (reader, writer) = client.split();

        let buf_reader = BufReader::with_capacity(self.cap_size, reader);
        let buf_writer = BufWriter::with_capacity(self.cap_size, writer);

        self.process_request_path(buf_reader, buf_writer).await?;

        Ok(())
    }
}
