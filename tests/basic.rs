use list_features::test_parse_feature_keys_from_lines as parse_feature_keys_from_lines;

fn parse_from_str(s: &str) -> std::collections::HashSet<String> {
  parse_feature_keys_from_lines(s.lines().map(str::to_string))
}

#[test]
fn single_feature() {
  let toml = r#"
    [features]
    default = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 1);
  assert!(features.contains("default"));
}

#[test]
fn multiple_features_with_comments() {
  let toml = r#"
    [features]
    # default feature
    default = ["foo"] # comment
    foo = []
    bar = ["baz", "qux"]
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 3);
  assert!(features.contains("default"));
  assert!(features.contains("foo"));
  assert!(features.contains("bar"));
}

#[test]
fn ignores_other_sections() {
  let toml = r#"
    [features]
    a = []
    b = []

    [dependencies]
    c = "1.0"
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 2);
  assert!(features.contains("a"));
  assert!(features.contains("b"));
  assert!(!features.contains("c"));
}

#[test]
fn ignores_multiline_values() {
  let toml = r#"
    [features]
    big = [
      "one",
      "two",
    ]
    small = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 2);
  assert!(features.contains("big"));
  assert!(features.contains("small"));
}
