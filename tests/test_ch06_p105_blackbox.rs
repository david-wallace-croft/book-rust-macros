use ::book_rust_macros::BuilderBlackbox;

#[test]
fn should_generate_builder_for_struct_with_no_properties() {
  #[derive(BuilderBlackbox)]
  struct ExampleStructNoFields {}

  let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
}

#[test]
fn should_generate_builder_for_struct_with_one_property() {
  #[derive(BuilderBlackbox)]
  struct Gleipnir {
    roots_of: String,
  }

  let gleipnir = Gleipnir::builder()
    .roots_of("mountains".to_string())
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
}
