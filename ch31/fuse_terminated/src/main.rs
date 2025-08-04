use std::time::Duration;

use futures::{
    FutureExt, Stream, StreamExt,
    future::{Fuse, FusedFuture},
    pin_mut, select,
    stream::FusedStream,
};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

async fn get_new_num() -> u8 {
    println!("Generating new number...");

    42
}

async fn run_on_new_num(num: u8) {
    println!("Running task with number: {num}");
}

async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();

    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() =>
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                },
            new_num = get_new_num_fut =>
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse()),
            () = run_on_new_num_fut =>
                println!("Completed processing number"),
            complete => panic!("`internal_timer` ompleted unexpectedly"),
        }
    }
}

#[tokio::main]
async fn main() {
    let interval = interval(Duration::from_secs(1));
    let interval_timer = IntervalStream::new(interval).map(|_| ()).fuse();

    run_loop(interval_timer, 10).await;
}
