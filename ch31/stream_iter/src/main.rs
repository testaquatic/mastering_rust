use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("example.com:80").await?;

    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n")
        .await?;
    stream.flush().await?;

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("Received: {}", line);
    }

    Ok(())
}
