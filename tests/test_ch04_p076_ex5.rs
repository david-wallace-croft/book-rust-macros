use book_rust_macros::public_ex5;

#[public_ex5]
struct MyStructNamed {
  alpha: u32,
  beta: String,
}

#[public_ex5]
struct MyStructUnnamed(u32, String);

#[public_ex5]
enum MyEnum {
  Alpha(u32),
  Beta(String),
}

#[test]
fn test_ch04_p076_ex5() {}
