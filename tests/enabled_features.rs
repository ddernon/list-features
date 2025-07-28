use std::env::{set_var, remove_var};
use list_features::test_list_enabled_among as list_enabled_among;

fn make_features(keys: &[&str]) -> std::collections::HashSet<String> {
  keys.iter().map(|s| s.to_string()).collect()
}

#[test]
fn detects_enabled_features() {
  let all = make_features(&["foo-bar", "baz"]);

  set_var("CARGO_FEATURE_FOO_BAR", "1");
  set_var("CARGO_FEATURE_UNUSED", "1");

  let result = list_enabled_among(&all);

  assert_eq!(result.len(), 1);
  assert!(result.contains(&String::from("foo-bar")));

  remove_var("CARGO_FEATURE_FOO_BAR");
  remove_var("CARGO_FEATURE_UNUSED");
}
