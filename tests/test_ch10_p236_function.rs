use ::book_rust_macros::config;

#[test]
fn configures() {
  config!(path = "tests/test_ch10_p236_function.yaml");

  let config: Config = Config::new();

  let user: &String = config.0.get("user").unwrap();

  assert_eq!(user, "admin");
}
