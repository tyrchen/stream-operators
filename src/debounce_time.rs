use crate::state::State;
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    task::{ready, Context, Poll},
    time::Duration,
};
use tokio::time::interval;
use tokio_stream::{wrappers::IntervalStream, Stream};

pin_project! {
    #[derive(Debug)]
    pub struct DebounceTime<S: Stream> {
        #[pin]
        stream: S,
        #[pin]
        interval: IntervalStream,
        item: Option<S::Item>,
        state: State,
    }
}

impl<S: Stream> DebounceTime<S> {
    pub fn new(stream: S, timeout: Duration) -> Self {
        Self {
            stream,
            interval: IntervalStream::new(interval(timeout)),
            item: None,
            state: State::HasNext,
        }
    }
}

impl<S: Stream> Stream for DebounceTime<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if *this.state == State::HasNext {
            match this.stream.poll_next(cx) {
                Poll::Ready(Some(item)) => {
                    *this.item = Some(item);
                }
                Poll::Ready(None) => {
                    *this.state = State::HasNone;
                }
                Poll::Pending => {}
            }
        }

        match this.state {
            State::HasNext => {
                let t = ready!(this.interval.poll_next(cx));
                println!("t: {:?}", t);
                if let Some(item) = this.item.take() {
                    Poll::Ready(Some(item))
                } else {
                    Poll::Pending
                }
            }
            State::HasNone => {
                if let Some(item) = this.item.take() {
                    cx.waker().wake_by_ref();
                    Poll::Ready(Some(item))
                } else {
                    *this.state = State::Done;
                    Poll::Ready(None)
                }
            }
            State::Done => panic!("poll_next called after completion"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::interval_value, StreamOps};
    use std::time::Duration;
    use tokio::time::interval;
    use tokio_stream::{wrappers::IntervalStream, StreamExt};

    #[tokio::test]
    async fn debounce_time_should_work() {
        let mut stream = interval_value(Duration::from_millis(1), 1, 1)
            .take(30)
            .debounce_time(Duration::from_millis(10));

        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(11));
        assert_eq!(stream.next().await, Some(21));
        assert_eq!(stream.next().await, Some(30));

        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn debounce_time_should_work_with_empty_stream() {
        let mut stream = IntervalStream::new(interval(Duration::from_millis(1)))
            .take(0)
            .debounce_time(Duration::from_millis(100));

        assert_eq!(stream.next().await, None);
    }
}
