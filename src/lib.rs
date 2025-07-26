use std::collections::HashSet;
use std::io::BufRead;


/// Returns the list of enabled features.
/// 
/// It reads from `std::env::vars` and filters against the features listed in the provided Cargo.toml file.
/// It should only be called in build scripts or code executed during a Cargo build.
/// Panics if the file can't be read.  /// # Arguments
///
/// * `cargo_file` - Path to the toml file to use as source for the available features list. Default: "Cargo.toml"
pub fn list_enabled_features(cargo_file: Option<&str>) -> Vec<String> {
  let path = cargo_file.unwrap_or("Cargo.toml");
  let all_features = list_all_features(path);
  list_enabled_features_among(&all_features)
}

/// Parses a Cargo.toml file and returns the set of declared feature names.
/// Only the `[features]` section is considered.
///
/// Panics if the file can't be read.
pub fn list_all_features(path: &str) -> HashSet<String> {
  let file = std::fs::File::open(path).unwrap_or_else(|_| {
    panic!("Cannot open {path}");
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
fn list_enabled_features_among(all_features: &std::collections::HashSet<String>) -> Vec<String> {
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
pub fn test_list_enabled_features_among(all_features: &std::collections::HashSet<String>) -> Vec<String> {
  list_enabled_features_among(all_features)
}
