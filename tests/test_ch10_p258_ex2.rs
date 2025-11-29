use ::book_rust_macros::config_ex2;
#[cfg(feature = "include")]
use ::std::collections::HashMap;

#[test]
fn configures() {
  #[config_ex2(path = "tests/test_ch10_p258_ex2.yaml")]
  struct MyConfig;

  let my_config: MyConfig = MyConfig::new();

  assert_eq!(my_config.username, "username4");
}

#[cfg(feature = "include")]
#[test]
fn configures_ex2() {
  #[config_ex2(path = "tests/test_ch10_p258_ex2.yaml")]
  struct MyConfig;

  let my_config: MyConfig = MyConfig::new();

  assert_eq!(my_config.username, "username4");

  #[config_ex2(path = "tests/test_ch10_p258_ex2.yaml")]
  struct MyConfig2;

  let my_config_2: MyConfig2 = MyConfig2::new();

  assert_eq!(my_config_2.password, "password4");

  let hash_map: HashMap<String, String> = my_config_2.into();

  assert_eq!(hash_map.get("username"), Some(&"username4".to_string()));
}
