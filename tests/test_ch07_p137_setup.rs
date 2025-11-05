use ::book_rust_macros::panic_to_result;

#[derive(Debug)]
pub struct Person {
  age: u32,
  name: String,
}

#[panic_to_result]
fn create_person(
  age: u32,
  name: String,
) -> Person {
  if age > 30 {
    panic!("Don't trust anyone over thirty.");
  }

  Person {
    age,
    name,
  }
}

#[test]
fn happy_path() {
  let actual: Person = create_person(22, "Sam".into());

  assert_eq!(actual.age, 22);

  assert_eq!(actual.name, "Sam");
}

#[test]
#[should_panic(expected = "Don't trust anyone over thirty.")]
fn should_panic_on_invalid_age() {
  let _actual: Person = create_person(32, "Sam".into());
}
