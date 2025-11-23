use ::book_rust_macros::iac_struct;

#[test]
fn parses() {
  iac_struct! {
    bucket uniquename => lambda (
      name = my_name,
      mem = 1024,
      time = 15,
    )
  }
}
