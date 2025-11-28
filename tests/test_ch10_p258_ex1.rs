use ::book_rust_macros::config_ex1;
use ::std::collections::HashMap;

#[test]
fn configures() {
  #[config_ex1(
    exclude = "from",
    path = "tests/test_ch10_p258_ex1.yaml"
  )]
  struct MyConfig;

  let my_config: MyConfig = MyConfig::new();

  assert_eq!(my_config.username, "username3");

  #[config_ex1(path = "tests/test_ch10_p258_ex1.yaml")]
  struct MyConfig2;

  let my_config_2: MyConfig2 = MyConfig2::new();

  assert_eq!(my_config_2.password, "password3");

  let hash_map: HashMap<String, String> = my_config_2.into();

  assert_eq!(hash_map.get("username"), Some(&"username3".to_string()));
}
