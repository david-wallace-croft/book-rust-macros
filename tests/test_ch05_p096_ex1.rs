use book_rust_macros::hello_world;

#[test]
fn test_ch05_p096_ex1() {
  hello_world!(
    struct Hello;
  );

  let hello: Hello = Hello;

  assert_eq!(hello.say_hello(), "Hello, World!");
}
