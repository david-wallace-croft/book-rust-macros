use book_rust_macros::HelloInput;

#[derive(HelloInput)]
enum Pet {
  Cat,
}

#[test]
fn test_ch03_p055_ex4() {
  let pet: Pet = Pet::Cat;

  let actual: String = pet.hello_input();

  assert_eq!(actual, "Hello, Pet!");
}
