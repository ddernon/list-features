use list_features::test_parse_feature_keys_from_lines as parse_feature_keys_from_lines;

fn parse_from_str(s: &str) -> std::collections::HashSet<String> {
  parse_feature_keys_from_lines(s.lines().map(str::to_string))
}

#[test]
fn ignores_lines_without_equals() {
  let toml = r#"
    [features]
    just_a_word
    foo = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 1);
  assert!(features.contains("foo"));
}

#[test]
fn ignores_commented_out_features() {
  let toml = r#"
    [features]
    #bar = []
    foo = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 1);
  assert!(features.contains("foo"));
  assert!(!features.contains("bar"));
}

#[test]
fn ignores_empty_keys() {
  let toml = r#"
    [features]
     = []
    foo = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 1);
  assert!(features.contains("foo"));
}

#[test]
fn strips_whitespace_from_keys() {
  let toml = r#"
    [features]
      foo   = []
      bar= []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 2);
  assert!(features.contains("foo"));
  assert!(features.contains("bar"));
}

#[test]
fn handles_duplicate_keys() {
  let toml = r#"
    [features]
    foo = []
    foo = ["something"]
    bar = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 2);
  assert!(features.contains("foo"));
  assert!(features.contains("bar"));
}

#[test]
fn accepts_multiple_features_sections() {
  let toml = r#"
    [features]
    foo = []
    bar = []

    [dependencies]
    baz = "1.0"

    [features] # Apparently that's allowed
    qux = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 3);
  assert!(features.contains("foo"));
  assert!(features.contains("bar"));
  assert!(features.contains("qux"));
}

#[test]
fn accept_dashes_and_underscores_and_caps() {
  let toml = r#"
    [features]
    name-with-dashes = []
    name_with_underscore = []
    SHOUTing = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 3);
  assert!(features.contains("name-with-dashes"));
  assert!(features.contains("name_with_underscore"));
  assert!(features.contains("SHOUTing"));
}

#[test]
fn tolerates_weird_spacing() {
  let toml = r#"
    [features]
    a=   []
    b =[]
    c    =     [ ]
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 3);
  assert!(features.contains("a"));
  assert!(features.contains("b"));
  assert!(features.contains("c"));
}

#[test]
fn skips_inline_sections() {
  let toml = r#"
    [features] # this is valid TOML
    foo = []
    [bad] foo = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 1);
  assert!(features.contains("foo"));
}

#[test]
fn no_features_section_yields_empty_set() {
  let toml = r#"
    [package]
    name = "example"
    version = "0.1.0"

    [dependencies]
    foo = "1.0"
  "#;

  let features = parse_from_str(toml);
  assert!(features.is_empty());
}

#[test]
fn accepts_quoted_feature_keys() {
  let toml = r#"
    [features]
    "foo" = []
    "bar-baz" = []
    "with space" = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 3);
  assert!(features.contains("foo"));
  assert!(features.contains("bar-baz"));
   // that one is actually not valid, but we’re not the compilation police
  assert!(features.contains("with space"));
}

#[test]
fn handles_mixed_quoted_and_unquoted_keys() {
  let toml = r#"
    [features]
    foo = []
    "bar-baz" = []
    baz = []
  "#;

  let features = parse_from_str(toml);
  assert_eq!(features.len(), 3);
  assert!(features.contains("foo"));
  assert!(features.contains("bar-baz"));
  assert!(features.contains("baz"));
}
