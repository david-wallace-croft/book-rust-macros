use ::book_rust_macros::BuilderFurther;

#[test]
fn should_generate_builder_for_struct_with_no_properties() {
  #[derive(BuilderFurther)]
  struct ExampleStructNoFields {}

  let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
}

#[test]
fn should_generate_builder_for_struct_with_one_property() {
  #[derive(BuilderFurther)]
  struct Gleipnir {
    roots_of: String,
  }

  let gleipnir = Gleipnir::builder()
    .roots_of("mountains".to_string())
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
}

#[test]
fn should_generate_builder_for_struct_with_two_properties() {
  #[derive(BuilderFurther)]
  struct Gleipnir {
    roots_of: String,
    breath_of_a_fish: u8,
  }

  let gleipnir = Gleipnir::builder()
    .roots_of("mountains".to_string())
    .breath_of_a_fish(1)
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
  assert_eq!(gleipnir.breath_of_a_fish, 1);
}
