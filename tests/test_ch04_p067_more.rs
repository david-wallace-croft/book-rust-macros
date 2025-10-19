use book_rust_macros::public_more;

#[public_more]
struct Example {
  first: String,
  pub second: u32,
}

#[test]
fn test_ch04_p067_more() {
  // empty
}
