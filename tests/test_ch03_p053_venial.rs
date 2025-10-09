use book_rust_macros::HelloVenial;

#[derive(HelloVenial)]
enum Pet {
  Cat,
}

#[test]
fn test_ch03_p053_venial() {
  let pet: Pet = Pet::Cat;

  let actual: String = pet.hello_venial();

  assert_eq!(actual, "Aloha, World!");
}
