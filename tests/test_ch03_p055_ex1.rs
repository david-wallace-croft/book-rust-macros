use book_rust_macros::UpperCaseName;

#[derive(UpperCaseName)]
struct Example;

#[test]
fn test_ch03_p055_ex1() {
  let example: Example = Example;

  let actual: String = example.uppercase();

  assert_eq!(actual, "EXAMPLE");
}
