use book_rust_macros::private_ex2;

private_ex2!(
  struct Example {
    pub number_value: i32,
    pub string_value: String,
  }
);

#[test]
fn test_ch05_p083_generating() {
  let example: Example = Example::new(2, "value".to_string());

  assert_eq!(example.get_number_value(), &2);

  assert_eq!(example.get_string_value(), "value");
}
