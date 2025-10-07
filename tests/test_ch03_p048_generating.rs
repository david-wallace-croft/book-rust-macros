use book_rust_macros::Hello;

#[derive(Hello)]
enum Pet {
  Cat,
}

#[test]
fn test_ch03_p048_generating() {
  let pet: Pet = Pet::Cat;

  let actual: String = pet.hello_world();

  assert_eq!(actual, "Hello, World!");
}
