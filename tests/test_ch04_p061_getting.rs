use book_rust_macros::public_getting;

#[public_getting]
struct Example {
  first: String,
  pub second: u32,
}

#[test]
fn test_ch04_p061_getting() {
  // empty
}
