use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8000").await?;

    stream.write_all(b"Hello, Server!").await?;
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
