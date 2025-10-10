use book_rust_macros::UpperCaseName;

#[derive(UpperCaseName)]
struct Example;

#[test]
fn test_ch03_p055_ex3() {
  let actual: String = Example::testing_testing();

  assert_eq!(actual, "one two three");
}
