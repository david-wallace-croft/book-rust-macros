#![allow(dead_code)]
#![allow(unused_macros)]

fn add_one(n: i32) -> i32 {
  n + 1
}

fn compose_two<FIRST, SECOND, THIRD, F, G>(
  f: F,
  g: G,
) -> impl Fn(FIRST) -> THIRD
where
  F: Fn(FIRST) -> SECOND,
  G: Fn(SECOND) -> THIRD,
{
  move |x| g(f(x))
}

fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
  move |x| format!("{}{}", prefix, x)
}

fn stringify(n: i32) -> String {
  n.to_string()
}

macro_rules! compose {
  ($last:expr) => { $last };

  ($head:expr, $($tail:expr),+) => {
    compose_two($head, compose!($($tail),+))
  }
}

macro_rules! compose_alt {
  ($last:expr) => { $last };

  ($head:expr => $($tail:expr) =>+) => {
    compose_two($head, compose_alt!($($tail) =>+))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use tracing::info;

  #[test]
  fn test1() {
    crate::init_tracing();

    let two_composed_function =
      compose_two(compose_two(add_one, stringify), prefix_with("Result: "));

    let result: String = two_composed_function(0);

    assert_eq!(result, "Result: 1");

    info!("{result}");
  }

  #[test]
  fn test2() {
    crate::init_tracing();

    let composed = compose!(add_one, stringify, prefix_with("Result: "));

    let result: String = composed(0);

    assert_eq!(result, "Result: 1");

    info!("{result}");
  }

  #[test]
  fn test3() {
    crate::init_tracing();

    let composed =
      compose_alt!(add_one => stringify => prefix_with("Result: "));

    let result: String = composed(0);

    assert_eq!(result, "Result: 1");

    info!("{result}");
  }
}
