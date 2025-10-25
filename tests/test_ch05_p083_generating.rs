use book_rust_macros::private_generating;

private_generating!(
  struct Example {
    number_value: i32,
    string_value: String,
  }
);

#[test]
fn test_ch05_p083_generating() {
  let example: Example = Example {
    number_value: 2,
    string_value: "value".to_string(),
  };

  assert_eq!(example.get_number_value(), &2);

  assert_eq!(example.get_string_value(), "value");
}
