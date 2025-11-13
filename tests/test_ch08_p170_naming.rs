use ::book_rust_macros::BuilderNaming;

#[test]
fn should_generate_builder_for_struct_with_no_properties() {
  #[derive(BuilderNaming)]
  struct ExampleStructNoFields {}

  let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
}

#[test]
fn should_generate_builder_for_struct_with_one_property() {
  #[derive(BuilderNaming)]
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
  #[derive(BuilderNaming)]
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
  #[derive(BuilderNaming)]
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
  #[derive(BuilderNaming)]
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
  #[derive(BuilderNaming)]
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
  #[derive(BuilderNaming)]
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
  #[derive(BuilderNaming)]
  struct Gleipnir {
    #[allow(dead_code)]
    roots_of: String,
  }

  Gleipnir::builder().build();
}
