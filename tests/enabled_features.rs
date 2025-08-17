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

#[test]
fn detects_enabled_features_with_underscore() {
  let all = make_features(&["under_scored", "dashed-name", "under_scored_and-dashed", "disabled_feature"]);

  set_var("CARGO_FEATURE_UNDER_SCORED", "1");
  set_var("CARGO_FEATURE_DASHED_NAME", "1");
  set_var("CARGO_FEATURE_UNDER_SCORED_AND_DASHED", "1");
  set_var("CARGO_FEATURE_UNUSED", "1");

  let result = list_enabled_among(&all);

  assert_eq!(result.len(), 3);
  assert!(result.contains(&String::from("under_scored")));
  assert!(result.contains(&String::from("dashed-name")));
  assert!(result.contains(&String::from("under_scored_and-dashed")));

  remove_var("CARGO_FEATURE_UNDER_SCORED");
  remove_var("CARGO_FEATURE_DASHED_NAME");
  remove_var("CARGO_FEATURE_UNDER_SCORED_AND_DASHED");
  remove_var("CARGO_FEATURE_UNUSED");
}

#[test]
fn detects_enabled_features_with_caps() {
  let all = make_features(&["Not_a-Good_name"]);

  set_var("CARGO_FEATURE_NOT_A_GOOD_NAME", "1");

  let result = list_enabled_among(&all);

  assert_eq!(result.len(), 1);
  assert!(result.contains(&String::from("Not_a-Good_name")));

  remove_var("CARGO_FEATURE_NOT_A_GOOD_NAME");
}
