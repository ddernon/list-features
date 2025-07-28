use std::collections::HashSet;
use std::io::BufRead;
use std::fmt::Write;


/// Returns the list of enabled features as a `Vec<String>`.
/// 
/// Reads from `std::env::vars` and filters the features based on those listed in the specified `Cargo.toml` file.
/// This function should only be called in build scripts or code executed during a Cargo build process.
/// 
/// Unless its output format doesn’t suit you, you’ll probably want to use [list_enabled_as_string] instead.
/// 
/// # Panics
/// 
/// Panics if the file at `cargo_toml_path` cannot be read.
/// 
/// # Arguments
///
/// * `cargo_toml_path` - Optional path to the `Cargo.toml` file used as the source for the available features list.
///  If `None` provided, defaults to `"Cargo.toml"``.
pub fn list_enabled(cargo_toml_path: Option<&str>) -> Vec<String> {
  let cargo_toml_path = cargo_toml_path.unwrap_or("Cargo.toml");
  let all_features = list_all(cargo_toml_path);
  list_enabled_among(&all_features)
}

/// Returns the list of enabled features as a `String`.
/// 
/// It’s a wrapper around [list_enabled] that provides a `String` that should be usable as-is in an output file of the build script.
/// The output format is:
/// ```rust
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
/// ## Arguments
///
/// * `const_name` - Name of the contant to be written ("CONST_NAME" in the example above).
/// * `cargo_toml_path` - Optional path to the `Cargo.toml` file used as the source for the available features list.
///  If `None` provided, defaults to `"Cargo.toml"`.
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
/// Only the `[features]` section is considered.
///
/// Panics if the file can't be read.
/// 
/// ## Arguments
///
/// * `cargo_toml_path` - Path to the toml file to use as source for the available features list.
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

/// Returns the list of enabled features that are present in `all_features`.
///
/// This reads from `std::env::vars` and filters against the provided set.
/// It should only be called in build scripts or code executed during a Cargo build.
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
