# Learning How to Read RSS

Goals:
- Get a list of RSS feeds from TOML and pull in the headlines
- Do so in a TUI
- Make a WASM version with a WUI

## Library survey

Quick search over [lib.rs](https://lib.rs) and one good potential candidate is
[feed-rs](https://lib.rs/crates/feed-rs). Other libraries I've found focus on
RSS serialization.