//! Returns the list of enable features when building your crate.
//!
//! The [`list_enabled_as_string`] function lists enabled features during the Cargo build process,
//! in a format that can be directly saved in build artifacts, which can be then included
//! elsewhere in the program and read at run time.
//! 
//! # Examples
//!
//! See the example included with the [`list_enabled_as_string`] function.


use std::collections::HashSet;
use std::io::BufRead;
use std::fmt::Write;


/// Returns the list of enabled features as a `Vec<String>`.
/// 
/// Reads from `std::env::vars` and filters the features based on those listed in the specified `Cargo.toml` file.
/// This function should only be called in build scripts or code executed during a Cargo build process, as
/// the required `CARGO_FEATURE_*` environment variables will be missing otherwise.
/// 
/// Unless its output format doesn’t suit you, you’ll probably want to use [`list_enabled_as_string`] instead.
/// 
/// # Panics
/// 
/// Panics if the specified file cannot be read.
/// 
/// # Arguments
///
/// * `cargo_toml_path` - Optional path to the `Cargo.toml` file used as the source for the available features list.
///   If `None` provided, defaults to `"Cargo.toml"``.
/// 
/// # Returns
///
/// A `Vec<String>` containing the names of the declared features, ordered with `default` first and then sorted alphabetically.
pub fn list_enabled(cargo_toml_path: Option<&str>) -> Vec<String> {
  let cargo_toml_path = cargo_toml_path.unwrap_or("Cargo.toml");
  let all_features = list_all(cargo_toml_path);
  list_enabled_among(&all_features)
}

/// Returns the list of enabled features as a `String`.
/// 
/// It’s a wrapper around [`list_enabled`] that provides a `String` that should be usable as is in an output file of the build script.
/// This function should only be called in build scripts or code executed during a Cargo build process, as
/// the required `CARGO_FEATURE_*` environment variables will be missing otherwise.
/// 
/// The output is string containing something like:
/// ```
/// pub const CONST_NAME: &[&str] = &[
/// "feature1",
/// "feature2",
/// ];
/// ```
/// 
/// # Panics
/// 
/// Panics if the file at `cargo_toml_path` cannot be read.
/// 
/// # Arguments
///
/// * `const_name` - Name of the constant to be written ("CONST_NAME" in the example above, "ENABLED_FEATURES" in the example below).
/// * `cargo_toml_path` - Optional path to the `Cargo.toml` file used as the source for the available features list.
///   If `None` provided, defaults to `"Cargo.toml"`.
/// 
/// # Examples
///
/// ```ignore
/// // in build.rs
/// let out_dir = std::env::var("OUT_DIR").unwrap();
/// let file_path = format!("{out_dir}/build_info.rs");
/// let features = list_features::list_enabled_as_string("ENABLED_FEATURES", None);
/// std::fs::write(file_path, features).unwrap();
///
/// // in main.rs
/// include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
/// for feature in ENABLED_FEATURES {
///   println!(output, " {feature}");
/// }
/// ```
pub fn list_enabled_as_string(const_name: &str, cargo_toml_path: Option<&str>) -> String {
  let enabled_features = list_enabled(cargo_toml_path);
  let mut buf = String::new();
  writeln!(buf, "pub const {const_name}: &[&str] = &[").unwrap();
  for feature in enabled_features {
    writeln!(buf, r#""{feature}","#).unwrap();
  }
  writeln!(buf, "];").unwrap();
  buf
}

/// Parses a Cargo.toml file and returns the set of declared feature names.
/// 
/// Only the `[features]` section is considered. While it should be able handle reasonable edge cases, this function also tries to
/// keep things simple and is not a replacement for a full [toml parser](https://crates.io/crates/toml).
///
/// # Panics
/// 
/// Panics if the specified file cannot be read.
/// 
/// # Arguments
///
/// * `cargo_toml_path` - Path to the `Cargo.toml` file used as the source for the available features list.
/// 
/// # Returns
///
/// A `HashSet<String>` containing the names of the declared features.
pub fn list_all(cargo_toml_path: &str) -> HashSet<String> {
  let file = std::fs::File::open(cargo_toml_path).unwrap_or_else(|_| {
    panic!("Cannot open {cargo_toml_path}");
  });
  let reader = std::io::BufReader::new(file);

  let lines = reader.lines().map(|line| {
    line.expect("Could not read line")
  });

  parse_feature_keys_from_lines(lines)
}

// Core parser logic that works on any line iterator.
fn parse_feature_keys_from_lines<I>(lines: I) -> HashSet<String>
where
  I: IntoIterator<Item = String>,
{
  let mut in_features = false;
  let mut features = HashSet::new();

  for line in lines {
    let stripped = line.split('#').next().unwrap_or("").trim();

    if stripped.starts_with('[') {
      in_features = stripped == "[features]";
      continue;
    }

    if in_features && !stripped.is_empty() {
      if let Some((key, _)) = stripped.split_once('=') {
        let key = key.trim().trim_matches('"');
        if !key.is_empty() {
          features.insert(key.to_string());
        }
      }
    }
  }

  features
}
#[cfg(feature = "test")]
pub fn test_parse_feature_keys_from_lines<I>(lines: I) -> HashSet<String>
where
  I: IntoIterator<Item = String>,
{
  parse_feature_keys_from_lines(lines)
}

// Returns the list of enabled features that are present in `all_features`.
//
// This reads from `std::env::vars` and filters against the provided set.
// It should only be called in build scripts or code executed during a Cargo build.
fn list_enabled_among(all_features: &std::collections::HashSet<String>) -> Vec<String> {
  let mut enabled: Vec<String> = std::env::vars()
    .filter_map(|(k, _)| {
      if let Some(name) = k.strip_prefix("CARGO_FEATURE_") {
        let normalized = name.to_lowercase().replace('_', "-");
        if all_features.contains(&normalized) {
          return Some(normalized);
        }
      }
      None
    })
    .collect();
  
  // reorder and put default at front
  enabled.sort();
  if let Some(pos) = enabled.iter().position(|f| f == "default") {
    let default_feature = enabled.remove(pos);
    enabled.insert(0, default_feature);
  }

  enabled
}
#[cfg(feature = "test")]
pub fn test_list_enabled_among(all_features: &std::collections::HashSet<String>) -> Vec<String> {
  list_enabled_among(all_features)
}
