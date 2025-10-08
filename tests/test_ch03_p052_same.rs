use book_rust_macros::HelloAlt;

#[derive(HelloAlt)]
enum Pet {
  Cat,
}

#[test]
fn test_ch03_p052_same() {
  let pet: Pet = Pet::Cat;

  let actual: String = pet.hello_alt();

  assert_eq!(actual, "Howdy, World!");
}
