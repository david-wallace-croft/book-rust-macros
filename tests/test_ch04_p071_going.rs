use book_rust_macros::public_going;

#[public_going]
struct Example {
  first: String,
  pub second: u32,
}

#[test]
fn test_ch04_p069_going() {
  // empty
}
