#[cfg(feature = "struct")]
use ::book_rust_macros::config_features;
#[cfg(feature = "struct")]
use ::std::collections::HashMap;

#[cfg(feature = "struct")]
#[test]
fn configures() {
  #[config_features(exclude = "from")]
  struct MyConfig;

  let my_config: MyConfig = MyConfig::new();

  assert_eq!(my_config.username, "username2");

  #[config_features]
  struct MyConfig2;

  let my_config_2: MyConfig2 = MyConfig2::new();

  assert_eq!(my_config.password, "password2");

  let hash_map: HashMap<String, String> = my_config_2.into();

  assert_eq!(hash_map.get("username"), Some(&"username2".to_string()));
}
