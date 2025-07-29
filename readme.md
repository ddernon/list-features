# list-features

Extracts the list of enabled feature flags during compilation. These flags can then be saved and displayed at run-time.

## Key Features
- "Double zero" dependency: This crate has no dependency and won’t, in its typical use case, be a run-time dependency.
- Low Rust version requirement: Compatible with Rust version 1.58.

If these characteristics are not of interest to you, the [built](https://crates.io/crates/built) or [toml](https://crates.io/crates/toml)
crates may be more appropriate for your needs. Otherwise, read on :)

## Note
This is a preview release, I’m still setting up stuff. It already should not change too much, but you might want to wait
for version 0.2+ if you want to be reasonably sure that a breaking change won’t soon follow.

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

## Windows 7 compatibility
The minimum required Rust version is 1.58. While I believe it shouldn’t change much in the foreseeable future, my main objective is to
remain at or below Rust 1.77, so as to keep Windows 7 compatibility.

For Rust 1.77 compatibility:
- change `version = 4` to `version = 3` in `Cargo.lock`
- then clippy and the tests can be run as follow:
```
cargo +1.77 clippy
cargo +1.77 test --features test
```
