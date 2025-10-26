use book_rust_macros::compose;

pub fn add_one(n: i32) -> i32 {
  n + 1
}

pub fn stringify(n: i32) -> String {
  n.to_string()
}

#[test]
fn test_ch05_p089_composing() {
  let composed = compose! { add_one . add_one . stringify };

  assert_eq!("12", composed(10));
}
