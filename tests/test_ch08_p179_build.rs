#![allow(non_camel_case_types)]

use ::book_rust_macros::BuilderBetter;

#[test]
fn should_generate_builder_for_struct_with_no_properties() {
  #[derive(BuilderBetter)]
  struct ExampleStructNoFields {}

  let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
}

#[test]
fn should_generate_builder_for_struct_with_one_property() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    roots_of: String,
  }

  let gleipnir: Gleipnir = Gleipnir::builder()
    .roots_of("mountains".to_string())
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
}

#[test]
fn should_generate_builder_for_struct_with_one_renamed_prop() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    #[rename = "tops_of"]
    roots_of: String,
  }

  let gleipnir: Gleipnir =
    Gleipnir::builder().tops_of("mountains".to_string()).build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
}

#[test]
fn should_generate_builder_for_struct_with_one_renamed_property() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    #[rename("tops_of")]
    roots_of: String,
  }

  let gleipnir: Gleipnir =
    Gleipnir::builder().tops_of("mountains".to_string()).build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
}

#[test]
fn should_generate_builder_for_struct_with_two_properties() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    roots_of: String,
    breath_of_a_fish: u8,
  }

  let gleipnir: Gleipnir = Gleipnir::builder()
    .roots_of("mountains".to_string())
    .breath_of_a_fish(1)
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
  assert_eq!(gleipnir.breath_of_a_fish, 1);
}

#[test]
fn should_generate_builder_for_struct_with_two_props_one_custom_name() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    #[rename("tops_of")]
    roots_of: String,
    breath_of_a_fish: u8,
  }

  let gleipnir: Gleipnir = Gleipnir::builder()
    .tops_of("mountains".to_string())
    .breath_of_a_fish(1)
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
  assert_eq!(gleipnir.breath_of_a_fish, 1);
}

#[test]
fn should_generate_builder_for_struct_with_multiple_properties() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    roots_of: String,
    breath_of_a_fish: u8,
    other_necessities: Vec<String>,
  }

  let gleipnir: Gleipnir = Gleipnir::builder()
    .roots_of("mountains".to_string())
    .breath_of_a_fish(1)
    .other_necessities(vec![
      "sound of cat's footsteps".into(),
      "beard of a woman".into(),
      "spittle of a bird".into(),
    ])
    .build();

  assert_eq!(gleipnir.roots_of, "mountains".to_string());
  assert_eq!(gleipnir.breath_of_a_fish, 1);
  assert_eq!(gleipnir.other_necessities.len(), 3);
}

#[test]
#[should_panic]
fn should_panic_when_field_is_missing() {
  #[derive(BuilderBetter)]
  struct Gleipnir {
    #[allow(dead_code)]
    roots_of: String,
  }

  Gleipnir::builder().build();
}

#[test]
fn should_use_defaults_when_attribute_is_present() {
  #[derive(BuilderBetter)]
  #[builder_defaults]
  struct ExampleStructTwoFields {
    int_value: i32,
    string_value: String,
  }

  let example: ExampleStructTwoFields =
    ExampleStructTwoFields::builder().build();

  assert_eq!(example.int_value, Default::default());

  assert_eq!(example.string_value, String::default());
}

#[test]
fn should_fail_to_compile_when_does_not_implement_defaults() {
  struct DoesNotImplementDefault;

  #[expect(dead_code)]
  #[derive(BuilderBetter)]
  // Unremark to see the compile error
  // #[builder_defaults]
  struct ExampleStruct {
    not: DoesNotImplementDefault,
  }
}
