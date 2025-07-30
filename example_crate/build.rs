fn main() {
  let out_dir = std::env::var("OUT_DIR").unwrap();
  let build_info_path = format!("{out_dir}/build_info.rs");
  let features = list_features::list_enabled_as_string("ENABLED_FEATURES");
  std::fs::write(build_info_path, features).unwrap();
  
  // force recompiling every time
  // (from https://stackoverflow.com/questions/49077147/how-can-i-force-build-rs-to-run-again-without-cleaning-my-whole-project)
  println!("cargo:rerun-if-changed=NULL");
}
