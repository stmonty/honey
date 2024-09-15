use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct CountFuture {
    count: usize,
    max: usize,
}

impl CountFuture {
    pub fn new(max: usize) -> Self {
        CountFuture { count: 0, max }
    }
}

impl Future for CountFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count >= self.max {
            Poll::Ready("Done")
        } else {
            self.count += 1;
            println!("Poll count: {}", self.count);
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    use std::pin::Pin;
    use std::future::Future;

    // Helper functions to create a dummy waker
    fn dummy_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            dummy_raw_waker()
        }

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    fn dummy_waker() -> Waker {
        unsafe { Waker::from_raw(dummy_raw_waker()) }
    }

    #[test]
    fn test_count_future() {
        let mut future = CountFuture::new(3);
        let waker = dummy_waker();
        let mut context = Context::from_waker(&waker);

        // First poll
        let mut pinned = Pin::new(&mut future);
        assert_eq!(pinned.as_mut().poll(&mut context), Poll::Pending);

        // Second poll
        assert_eq!(pinned.as_mut().poll(&mut context), Poll::Pending);

        // Third poll
        assert_eq!(pinned.as_mut().poll(&mut context), Poll::Pending);

        // Fourth poll should be Ready
        assert_eq!(pinned.as_mut().poll(&mut context), Poll::Ready("Done"));
    }
}

