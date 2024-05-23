use std::error::Error;
use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }

    pub async fn read_request<T: AsyncReadExt + Unpin>(
        &self,
        mut reader: BufReader<T>,
    ) -> io::Result<String> {
        let mut http_request = Vec::new();
        let n = reader.read_until(b'\n', &mut http_request).await?;
        let request_str = String::from_utf8_lossy(&http_request[..n])
            .trim_end()
            .to_string();
        Ok(request_str)
    }

    pub async fn write_response<T: AsyncWriteExt + Unpin>(
        &self,
        mut writer: BufWriter<T>,
        context: &str,
    ) -> io::Result<()> {
        writer.write_all(context.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }

    pub async fn process_client(&self, mut client: TcpStream) -> Result<(), Box<dyn Error>> {
        let (reader, writer) = client.split();

        let buf_reader = BufReader::with_capacity(1024, reader);
        let buf_writer = BufWriter::with_capacity(1024, writer);

        let req = self.read_request(buf_reader).await?;
        self.process_request_path(req, buf_writer).await?;

        Ok(())
    }

    pub async fn process_request_path<T: AsyncWriteExt + Unpin>(
        &self,
        request: String,
        writer: BufWriter<T>,
    ) -> io::Result<()> {
        let request_parts: Vec<&str> = request.split_whitespace().collect();
        let method = request_parts[0];
        let path = request_parts[1];

        if method == "GET" {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", path.len(), path);
            self.write_response(writer, &response).await?;
        }
        Ok(())
    }
}
