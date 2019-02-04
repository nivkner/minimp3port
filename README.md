This is a port of [minimp3](https://github.com/lieff/minimp3) to [The Rust Programming Language](https://github.com/rust-lang/rust),
inspired by [rinimp3](https://github.com/icefoxen/rinimp3).

The bulk of the port was done by auto-generating rust code using [c2rust](https://github.com/immunant/c2rust),
which was later rewritten incrementally to be more idiomatic.

The scripts used for integration tests in the original were replaced with rust-style integration tests, and can be run with `cargo test`.

The project contains a subcrate which is used to fuzz the main crate using [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).
To fuzz the crate, do the following:
```
cargo install honggfuzz
cd mp3fuzz
cargo hfuzz run mp3fuzz
```

The project supports `no_std` and uses no runtime dependencies.

**Warning**
---------
the project is not actively maintained, please do not use in production.
