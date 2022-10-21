# Stream Operators

This is a collection of stream operators for rust/tokio. Though the operators are coming from the [ReactiveX](http://reactivex.io/) world, I'm not trying to reinvent the observable ecosystem for rust. If you need that, you could use [rxrust](https://docs.rs/rxrust/latest/rxrust/). For this repom I'm just trying to provide a set of stream operators which could be used for [Stream](https://docs.rs/futures/latest/futures/stream/trait.Stream.html).

## Supported Operators

- [x] [debounce_time](https://rxjs.dev/api/operators/debounceTime)
- [x] [distinct_until_changed](https://rxjs.dev/api/operators/distinctUntilChanged)
- [x] [distinct](https://rxjs.dev/api/operators/distinct)
- [ ] [buffer_count](https://rxjs.dev/api/operators/bufferCount)
- [ ] [buffer_time](https://rxjs.dev/api/operators/bufferTime)

## Usage

See tests for each operator.
