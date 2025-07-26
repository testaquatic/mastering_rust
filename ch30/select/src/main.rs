use std::thread;

use crossbeam_channel::{select, unbounded};

fn main() {
    let (sender1, receiver1) = unbounded();
    let (sender2, receiver2) = unbounded();

    thread::spawn(move || {
        sender1.send("From channel 1").unwrap();
    });

    thread::spawn(move || sender2.send("From channel 2").unwrap());

    select! {
        recv(receiver1) -> msg => println!("{}", msg.unwrap()),
        recv(receiver2) -> msg => println!("{}", msg.unwrap()),
    }
}
