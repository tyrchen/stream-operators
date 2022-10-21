# Stream Operators

This is a collection of selected rxjs operators for rust standard Stream. Though the operators are coming from the [ReactiveX](http://reactivex.io/) world, I'm not trying to reinvent the observable ecosystem for rust. If you need that, you could use [rxrust](https://docs.rs/rxrust/latest/rxrust/). For this repom I'm just trying to provide a set of stream operators which could be used for [Stream](https://docs.rs/futures/latest/futures/stream/trait.Stream.html).

## Supported Operators

- [x] [debounce_time](https://rxjs.dev/api/operators/debounceTime)
- [x] [distinct_until_changed](https://rxjs.dev/api/operators/distinctUntilChanged)
- [x] [distinct](https://rxjs.dev/api/operators/distinct)
- [ ] [pluck](https://rxjs.dev/api/operators/pluck): Maps each source value to its specified nested property.
- [ ] [find](https://rxjs.dev/api/operators/find): Emits the first value that matches the condition.
- [ ] [find_index](https://rxjs.dev/api/operators/findIndex): Emits the index of the first value that matches the condition.
- [ ] [group_by](https://rxjs.dev/api/operators/groupBy): Groups the items emitted by an Observable according to a specified criterion, and emits these grouped items as GroupedObservables, one GroupedObservable per group.
- [ ] [pairwise](https://rxjs.dev/api/operators/pairwise): Emits the previous and current value as a two-element array.
- [ ] [partition](https://rxjs.dev/api/operators/partition): Splits the source Observable into two, one with values that satisfy a predicate, and another with values that don't satisfy the predicate.
- [ ] [sample_time](https://rxjs.dev/api/operators/sampleTime): Samples the source Observable at periodic time intervals, emitting the most recent value emitted by the source Observable since the previous sampling, unless the source has not emitted anything since the previous sampling.
- [ ] [sequence_equal](https://rxjs.dev/api/operators/sequenceEqual): Determines whether two Observables emit the same sequence of items.
- [ ] [throttle_time](https://rxjs.dev/api/operators/throttleTime): Emits a value from the source Observable, then ignores subsequent source values for a duration determined by another Observable, then repeats this process.

### Operators supported by tokio-stream

- [x] [every](https://rxjs.dev/api/operators/every): see [tokio-stream::StreamExt::all](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.all)
- [x] [filter](https://rxjs.dev/api/operators/filter): see [tokio-stream filter](https://docs.rs/tokio-stream/0.1.7/tokio_stream/trait.StreamExt.html#method.filter)
- [x] [map](https://rxjs.dev/api/operators/map): see [tokio-stream map](https://docs.rs/tokio-stream/0.1.7/tokio_stream/trait.StreamExt.html#method.map)
- [x] [reduce](https://rxjs.dev/api/operators/reduce): see [tokio-stream fold](https://docs.rs/tokio-stream/0.1.7/tokio_stream/trait.StreamExt.html#method.fold)
- [x] [concat_with](https://rxjs.dev/api/operators/concatWith): see [tokio-stream chain](https://docs.rs/tokio-stream/0.1.7/tokio_stream/trait.StreamExt.html#method.chain)
- [x] [buffer_time](https://rxjs.dev/api/operators/bufferTime): see [tokio-stream chunks_timeout](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.chunks_timeout)
- [x] [buffer_count](https://rxjs.dev/api/operators/bufferCount): see [tokio-stream chunks_timeout](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.chunks_timeout)
- [x] [merge_with](https://rxjs.dev/api/operators/mergeWith): see [tokio-stream merge](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge)
- [x] [skip](https://rxjs.dev/api/operators/skip): see [tokio-stream skip](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.skip)
- [x] [skip_while](https://rxjs.dev/api/operators/skipWhile): see [tokio-stream skip_while](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.skip_while)
- [x] [take](https://rxjs.dev/api/operators/take): see [tokio-stream take](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.take)
- [x] [take_while](https://rxjs.dev/api/operators/takeWhile): see [tokio-stream take_while](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.take_while)
- [x] [timeout](https://rxjs.dev/api/operators/timeout): see [tokio-stream timeout](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.timeout)
- [x] [delay](https://rxjs.dev/api/operators/delay): see [tokio-stream throttle](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.throttle)

### Operators supported by futures

- [x] [count](https://rxjs.dev/api/operators/count): see [futures::StreamExt::count](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.count)
- [x] [flat_map](https://rxjs.dev/api/operators/flatMap): see [futures::StreamExt::flat_map](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.flat_map)
- [ ] [repeat](https://rxjs.dev/api/operators/repeat): see [futures::StreamExt::cycle](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.cycle)
- [x] [scan](https://rxjs.dev/api/operators/scan): see [futures scan](https://docs.rs/futures/0.3.17/futures/stream/trait.StreamExt.html#method.scan)
- [x] [zip_with](https://rxjs.dev/api/operators/zipWith): see [futures zip](https://docs.rs/futures/0.3.17/futures/stream/trait.StreamExt.html#method.zip)

## Usage

See tests for each operator.
