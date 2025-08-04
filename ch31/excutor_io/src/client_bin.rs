use std::{
    net::SocketAddr,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Context,
};

use excutor_io::client::{Client, ReadFuture};
use futures::task;
use mio::{Events, Interest, Poll, Token, net::TcpStream};

fn main() {
    let addr = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();

    let poll = Arc::new(Mutex::new(Poll::new().unwrap()));
    let mut events = Events::with_capacity(1024);

    poll.lock()
        .unwrap()
        .registry()
        .register(
            &mut stream,
            mio::Token(0),
            Interest::READABLE | Interest::WRITABLE,
        )
        .unwrap();

    let mut client = Client::new(stream, Arc::clone(&poll));

    let data = b"Hello, Server!";
    client.send_data(data).unwrap();

    loop {
        poll.lock().unwrap().poll(&mut events, None).unwrap();

        for event in events.iter() {
            if let Token(0) = event.token() {
                if event.is_readable() {
                    let mut read_future = ReadFuture {
                        client: &mut client,
                    };

                    let waker = futures::task::noop_waker();
                    let mut cx = Context::from_waker(&waker);

                    match Pin::new(&mut read_future).poll(&mut cx) {
                        task::Poll::Ready(Ok(data)) => {
                            println!("Received: {:?}", String::from_utf8_lossy(&data));
                            return;
                        }
                        task::Poll::Pending => println!("Waiting for data..."),
                        task::Poll::Ready(Err(e)) => eprintln!("Error: {e}"),
                    }
                }
            }
        }
    }
}
