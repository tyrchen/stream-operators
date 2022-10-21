use crate::{state::State, ItemKey};
use pin_project_lite::pin_project;
use std::{
    collections::HashSet,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio_stream::Stream;

pin_project! {
    #[derive(Debug)]
    pub struct Distinct<S> where S: Stream, S::Item: ItemKey {
        #[pin]
        stream: S,
        items: HashSet<<<S as Stream>::Item as ItemKey>::Key>,
        state: State,
    }
}

impl<S> Distinct<S>
where
    S: Stream,
    S::Item: ItemKey,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            items: HashSet::default(),
            state: State::HasNext,
        }
    }
}

impl<S> Stream for Distinct<S>
where
    S: Stream,
    S::Item: ItemKey,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if *this.state == State::HasNext {
            let next = ready!(this.stream.poll_next(cx));
            if let Some(next) = next {
                let key = next.key();
                if !this.items.contains(&key) {
                    this.items.insert(key);
                    return Poll::Ready(Some(next));
                }
            } else {
                *this.state = State::HasNone;
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

macro_rules! impl_item_key {
    ($($t:ty),*) => {
        $(
            impl ItemKey for $t {
                type Key = Self;
                fn key(&self) -> Self::Key {
                    self.clone()
                }
            }
        )*
    };
}

impl_item_key!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_item_key!(String);

#[cfg(test)]
mod tests {
    use crate::{ItemKey, StreamOps};
    use tokio_stream::{iter, StreamExt};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Todo {
        id: u32,
        title: String,
    }

    impl Todo {
        fn new(id: u32, title: &str) -> Self {
            Self {
                id,
                title: title.to_string(),
            }
        }
    }

    impl ItemKey for Todo {
        type Key = u32;
        fn key(&self) -> Self::Key {
            self.id
        }
    }

    #[tokio::test]
    async fn distinct_should_work() {
        let mut stream = iter(vec![1, 1, 2, 2, 2, 1, 2, 3, 4, 3, 2, 1]).distinct();
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, Some(4));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn distinct_empty_should_work() {
        let mut stream = iter(1..1).distinct();
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn distinct_one_should_work() {
        let mut stream = iter(vec![1]).distinct();
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn distinct_with_todos_should_work() {
        let mut stream = iter(vec![
            Todo::new(1, "Buy milk"),
            Todo::new(2, "Buy eggs"),
            Todo::new(1, "Buy more milk"),
            Todo::new(3, "Buy bread"),
            Todo::new(2, "Buy more eggs"),
        ])
        .distinct();

        assert_eq!(stream.next().await, Some(Todo::new(1, "Buy milk")));
        assert_eq!(stream.next().await, Some(Todo::new(2, "Buy eggs")));
        assert_eq!(stream.next().await, Some(Todo::new(3, "Buy bread")));
        assert_eq!(stream.next().await, None);
    }
}
