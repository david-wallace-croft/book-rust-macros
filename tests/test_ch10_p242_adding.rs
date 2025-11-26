use ::book_rust_macros::config_struct;

#[test]
fn configures() {
  #[config_struct(path = "tests/test_ch10_p242_adding.yaml")]
  struct MyConfig;

  let my_config: MyConfig = MyConfig::new();

  assert_eq!(my_config.username, "username1");
}
