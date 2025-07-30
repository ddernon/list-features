// Run like:
// cargo run
// cargo run --no-default-features
// cargo run --no-default-features --features feature2


include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
  println!("Enabled features: {:?}", ENABLED_FEATURES);
}
