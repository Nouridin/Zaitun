use std::net::TcpStream;

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn connect(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;
        Ok(TcpClient { stream })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        self.stream.write(data)
    }
}