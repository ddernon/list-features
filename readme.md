# list-features

Extracts the list of enabled feature flags during compilation. These flags can then be saved and displayed at run-time.

## Highlights
- Zero dependencies: This crate has no dependency and won’t, in its typical use case, be a run-time dependency.
- Low Rust version requirement: Compatible with Rust version 1.58 (see also [Windows 7 compatibility](#windows-7-compatibility)).

If these characteristics are not important for your use case, the [built](https://crates.io/crates/built) or [toml](https://crates.io/crates/toml)
crates might be more appropriate for your needs. Otherwise, stick around :)

## Example
```rust
// In build.rs
let out_dir = std::env::var("OUT_DIR").unwrap();
let build_info_path = format!("{out_dir}/build_info.rs");
let features = list_features::list_enabled_as_string("ENABLED_FEATURES");
std::fs::write(build_info_path, features).unwrap();

// In main.rs
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
println!("Features: {:?}", ENABLED_FEATURES);
```
See also the [example crate](https://framagit.org/dder/list-features/-/tree/master/example_crate) and the docs.

## Windows 7 compatibility
The minimum required Rust version is 1.58. While this is unlikely to change in the foreseeable future,
the main objective is to remain at or below Rust 1.77, so as to preserve
[Windows 7 compatibility](https://blog.rust-lang.org/2024/02/26/Windows-7/).

To test with Rust 1.77:
- Change `version = 4` to `version = 3` in `Cargo.lock`.
- Install the 1.77 target: `rustup install 1.77.0-x86_64-pc-windows-gnu`.
- Then run clippy and the tests as follows:
```
cargo +1.77 clippy
cargo +1.77 test --features test
```
