use std::{
    io::{self, Read, Write},
    sync::{Arc, Mutex},
    task,
};

use mio::{Poll, net::TcpStream};

pub struct Client {
    stream: TcpStream,
    _poll: Arc<Mutex<Poll>>,
}

impl Client {
    pub fn new(stream: TcpStream, poll: Arc<Mutex<Poll>>) -> Self {
        Self {
            stream,
            _poll: poll,
        }
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), io::Error> {
        self.stream.write_all(data)
    }

    pub fn read_data(&mut self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![0; 1024];
        let n = self.stream.read(&mut buf)?;
        buf.truncate(n);

        Ok(buf)
    }
}

pub struct ReadFuture<'a> {
    pub client: &'a mut Client,
}

impl<'a> Future for ReadFuture<'a> {
    type Output = Result<Vec<u8>, io::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        let mut buf = vec![0; 1024];

        match this.client.stream.read(&mut buf) {
            Ok(0) => task::Poll::Ready(Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "EOF reached",
            ))),
            Ok(n) => {
                buf.truncate(n);
                task::Poll::Ready(Ok(buf))
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => task::Poll::Pending,
            Err(e) => task::Poll::Ready(Err(e)),
        }
    }
}
