use book_rust_macros::hello_world;

#[test]
fn test_ch05_p096_ex1() {
  struct Hello;

  hello_world!(Hello);

  let hello: Hello = Hello;

  assert_eq!(hello.say_hello(), "Hello, World!");
}
