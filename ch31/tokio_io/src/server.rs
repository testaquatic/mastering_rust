use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server running on 127.0.0.1:8000");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = vec![0; 1024];

        let n = socket.read(&mut buf).await?;
        println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

        socket.write_all(b"Hello, Client!").await?;
    }
}
