# Learning How to Read RSS

Goals:
- ~~Get a list of RSS feeds from TOML and pull in the headlines.~~
- Do so asynchronously.
- Good domain objects for output.
- Write a TUI application that shows the feeds, links, and text (in three
  columns).
- Make a WASM version with a WUI.

## Library survey

### Parsing Feeds
Quick search over [lib.rs](https://lib.rs) and one good potential candidate is
[feed-rs](https://lib.rs/crates/feed-rs). Other libraries I've found focus on
RSS serialization.

### Working With HTTP Requests

[isahc](https://lib.rs/crates/isahc) seems to be better than
[reqwest](https://lib.rs/crates/reqwest), judging by [lib.rs's](https://lib.rs)
`NOT WASM32` tag on their dependencies, although I'm not sure how relevant it
is. [isahc](https://lib.rs/crates/isahc) pulls in ca. 17MB of dependencies
(which seems dodgy), whereas [reqwest](https://lib.rs/crates/reqwest) requires
at most 10MB of explicit dependencies, plus ca. 2MB of
[tokio](https://lib.rs/crates/tokio) as a separate, mandatory dependency.