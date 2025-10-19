use book_rust_macros::public_parse;

#[public_parse]
struct Example {
  first: String,
  pub second: u32,
}

#[test]
fn test_ch04_p069_parse() {
  // empty
}
