use ::book_rust_macros::panic_to_result_changing;

#[derive(Debug)]
pub struct Person {
  age: u32,
  name: String,
}

#[panic_to_result_changing]
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
  let actual: Person = create_person(22, "Sam".into()).unwrap();

  assert_eq!(actual.age, 22);

  assert_eq!(actual.name, "Sam");
}

#[test]
fn should_err_on_invalid_age() {
  let actual: Result<Person, String> = create_person(32, "Sam".into());

  let error_message: String = actual.expect_err("this should be an err");

  assert_eq!(error_message, "Don't trust anyone over thirty.");
}
