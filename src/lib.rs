mod debounce_time;
mod dinstinct_until_changed;
mod distinct;
mod state;

use std::{fmt, time::Duration};
use tokio_stream::Stream;

pub use debounce_time::DebounceTime;
pub use dinstinct_until_changed::DistinctUntilChanged;
use distinct::Distinct;

pub trait StreamOps: Stream + Sized {
    fn debounce_time(self, timeout: Duration) -> DebounceTime<Self>;
    fn distinct(self) -> Distinct<Self>
    where
        Self::Item: ItemKey;
    fn distinct_until_changed(self) -> DistinctUntilChanged<Self>;
}

pub trait ItemKey {
    type Key: fmt::Debug + PartialEq + Eq + std::hash::Hash;
    fn key(&self) -> Self::Key;
}

impl<S> StreamOps for S
where
    S: Stream + Sized,
{
    fn debounce_time(self, timeout: Duration) -> DebounceTime<Self> {
        DebounceTime::new(self, timeout)
    }

    fn distinct(self) -> Distinct<Self>
    where
        Self::Item: ItemKey,
    {
        Distinct::new(self)
    }

    fn distinct_until_changed(self) -> DistinctUntilChanged<Self> {
        DistinctUntilChanged::new(self)
    }
}

#[cfg(test)]
pub mod test_utils {
    use futures::stream::StreamExt as _;
    use std::time::Duration;
    use tokio::time::interval;
    use tokio_stream::{wrappers::IntervalStream, Stream};

    pub fn interval_value(
        duration: Duration,
        start: usize,
        step: usize,
    ) -> impl Stream<Item = usize> {
        IntervalStream::new(interval(duration))
            .enumerate()
            .map(move |(i, _)| start + i * step)
    }
}
