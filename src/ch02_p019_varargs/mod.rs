#[allow(dead_code)]
fn base_greeting_fn(
  name: &str,
  greeting: &str,
) -> String {
  format!("{}, {}!", greeting, name)
}

#[allow(unused_macros)]
macro_rules! greeting {
  ($name:literal) => {
    base_greeting_fn($name, "Hello")
  };

  ($name:literal, $greeting:literal) => {
    base_greeting_fn($name, $greeting)
  };
}

#[cfg(test)]
mod test {
  use super::base_greeting_fn;
  use tracing::info;

  #[test]
  fn test1() {
    crate::init_tracing();

    let greet: String = greeting!("World", "Howdy");

    info!("{greet}");

    let greet_with_default: String = greeting!("World");

    info!("{greet_with_default}");
  }
}
