use std::{
    io::{Read, Write},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use mio::{Events, Interest, Poll, Token, net::TcpListener};

fn main() {
    let addr = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let mut listener = TcpListener::bind(addr).unwrap();

    let poll = Arc::new(Mutex::new(Poll::new().unwrap()));
    let mut events = Events::with_capacity(1024);

    poll.lock()
        .unwrap()
        .registry()
        .register(&mut listener, Token(0), Interest::READABLE)
        .unwrap();

    loop {
        poll.lock().unwrap().poll(&mut events, None).unwrap();

        for event in events.iter() {
            if let Token(0) = event.token() {
                if let Ok((mut stream, _)) = listener.accept() {
                    println!("Client connected");

                    let mut buf = vec![0; 1024];
                    match stream.read(&mut buf) {
                        Ok(0) => {
                            println!("Connection closed");
                            break;
                        }
                        Ok(n) => {
                            buf.truncate(n);
                            println!("Received: {:?}", String::from_utf8_lossy(&buf));
                            let response = b"Hello, Client";
                            stream.write_all(response).unwrap();
                        }
                        Err(e) => eprintln!("Error reading: {e}"),
                    }
                }
            }
        }
    }
}
