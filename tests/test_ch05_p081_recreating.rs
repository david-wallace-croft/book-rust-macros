use book_rust_macros::private;

private!(
  struct Example {
    number_value: i32,
    string_value: String,
  }
);

#[test]
fn test_ch05_p081_recreating() {
  let example: Example = Example {
    number_value: 2,
    string_value: "value".to_string(),
  };

  assert_eq!(example.number_value, 2);

  assert_eq!(example.string_value, "value");
}
