#[cfg(feature = "struct")]
use ::book_rust_macros::config_features;

#[cfg(feature = "struct")]
#[test]
fn configures() {
  #[config_features(path = "tests/test_ch10_p243_features.yaml")]
  struct MyConfig;

  let my_config: MyConfig = MyConfig::new();

  assert_eq!(my_config.username, "username2");
}
