use std::task::Poll;

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

struct DummyFuture {
    data: bool,
}

impl SimpleFuture for DummyFuture {
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.data {
            Poll::Ready(())
        } else {
            self.data = true;
            wake();
            Poll::Pending
        }
    }
}

fn main() {
    let mut join = Join {
        a: Some(DummyFuture { data: false }),
        b: Some(DummyFuture { data: false }),
    };

    let waker = || println!("Waker called");
    let mut attempts = 0;
    while join.poll(waker).is_pending() {
        attempts += 1;
        println!("Pending attempts {attempts}");
    }

    println!("Both futures completed");
}
