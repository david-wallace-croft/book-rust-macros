use ::book_rust_macros::Builder;

#[derive(Builder)]
struct Gleipnir {}

#[test]
fn test_ch06_p101_fleshing() {
  let _ = Gleipnir {};
}
