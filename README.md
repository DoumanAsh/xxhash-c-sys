# xxhash-c-sys

[![Rust](https://github.com/DoumanAsh/xxhash-c-sys/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/xxhash-c-sys/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/xxhash-c-sys.svg)](https://crates.io/crates/xxhash-c-sys)
[![Documentation](https://docs.rs/xxhash-c-sys/badge.svg)](https://docs.rs/crate/xxhash-c-sys/)

Proper bindings to [xxHash](https://github.com/Cyan4973/xxHash)

Current version corresponds to [0.8.3](https://github.com/Cyan4973/xxHash/releases/tag/v0.8.3)

## Features

- `no_std` - Disables C Std library usage in xxhash. Specifically it makes `malloc` fail
