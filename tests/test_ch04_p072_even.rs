use book_rust_macros::public_going;

#[public_going]
struct Example {
  first: String,
  pub second: u32,
}

#[test]
fn test_ch04_p072_even() {
  let example: Example = Example {
    first: "first".into(),
    second: 2,
  };

  assert_eq!(example.second, 2);
}
