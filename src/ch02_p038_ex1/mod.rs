#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! impl_hello_world {
  ($something:ident) => {
    impl $something {
      fn hello_world(&self) {
        info!("Hello, World!");
      }
    }
  };
}

#[cfg(test)]
mod test {
  // use super::*;
  use tracing::info;

  #[test]
  fn test1() {
    crate::init_tracing();

    struct Alpha;

    impl_hello_world!(Alpha);

    let a: Alpha = Alpha;

    a.hello_world();
  }
}
