# list-features

During compilation, extracts a list of enabled feature flags that you can then save and display at run-time.

"Double zero" dependency, as this crate 1) has no dependency and 2) won’t, in usual cases, be a run-time dependency.


## Example
```rust
// in build.rs
let out_dir = std::env::var("OUT_DIR").unwrap();
let file_path = format!("{out_dir}/build_info.rs");
let features = list_features::list_enabled_as_string("ENABLED_FEATURES", None);
std::fs::write(file_path, features).unwrap();

// in main.rs
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
for feature in ENABLED_FEATURES {
  println!(output, " {feature}");
}
```


## Windows 7 compatibility
I’m not sure of the minimum required Rust version. I took Rust 1.77 as a target so as to keep it Windows 7 compatible.

For Rust 1.77 compat (Win 7):
- change `version = 4` to `version = 3` in `Cargo.lock`
- then clippy and tests can be run as follow:
```
cargo +1.77 clippy
cargo +1.77 test --features test
```
