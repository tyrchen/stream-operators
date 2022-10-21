use crate::state::State;
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio_stream::Stream;

pin_project! {
    #[derive(Debug)]
    pub struct DistinctUntilChanged<S: Stream> {
        #[pin]
        stream: S,
        prev: Option<S::Item>,
        state: State,
    }
}

impl<S: Stream> DistinctUntilChanged<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            prev: None,
            state: State::HasNext,
        }
    }
}

impl<S> Stream for DistinctUntilChanged<S>
where
    S: Stream,
    S::Item: PartialEq,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if *this.state == State::HasNext {
            let next = ready!(this.stream.poll_next(cx));
            match (next, this.prev.take()) {
                (Some(next), Some(prev)) => {
                    if next == prev {
                        *this.prev = Some(next);
                    } else {
                        *this.prev = Some(next);
                        return Poll::Ready(Some(prev));
                    }
                }
                (Some(next), None) => {
                    *this.prev = Some(next);
                }
                (None, Some(prev)) => {
                    *this.state = State::HasNone;
                    return Poll::Ready(Some(prev));
                }
                (None, None) => {
                    *this.state = State::HasNone;
                }
            }
        }

        match this.state {
            State::HasNext => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::HasNone => {
                *this.state = State::Done;
                Poll::Ready(None)
            }
            State::Done => panic!("poll_next called after completion"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{test_utils::interval_value, StreamOps};
    use tokio_stream::{iter, StreamExt};

    #[tokio::test]
    async fn distinct_until_changed_should_work() {
        let mut stream = iter(vec![1, 1, 2, 3, 3, 3, 4, 4, 4, 4, 5]).distinct_until_changed();
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, Some(4));
        assert_eq!(stream.next().await, Some(5));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn distinct_until_changed_empty_should_work() {
        let mut stream = iter(1..1).distinct_until_changed();
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn distinct_until_changed_one_should_work() {
        let mut stream = iter(vec![1]).distinct_until_changed();
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn distinct_until_changed_with_interval_should_work() {
        let mut stream = interval_value(Duration::from_millis(1), 1, 0)
            .take(30)
            .distinct_until_changed();
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, None);
    }
}
