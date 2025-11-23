use ::book_rust_macros::iac_using;

#[test]
fn parses() {
  iac_using! {
    bucket uniquename => lambda (
      name = my_name,
      mem = 1024,
      time = 15,
    )
  }
}
