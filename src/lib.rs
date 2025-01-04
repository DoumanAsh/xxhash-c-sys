//!Proper bindings to [xxHash](https://github.com/Cyan4973/xxHash)
//!
//!Current version corresponds to [0.8.3](https://github.com/Cyan4973/xxHash/releases/tag/v0.8.3)
//!
//!## Features
//!
//!- `no_std` - Disables C Std library usage in xxhash. Specifically it makes `malloc` fail
#![no_std]

#[allow(warnings)]
mod bindings;
pub use bindings::*;
