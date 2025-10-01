#[allow(unused_macros)]
macro_rules! my_vec {
  () => [
    Vec::new()
  ];

  (make an empty vec) => (
    Vec::new()
  );

  {$x:expr} => {
    {
      let mut v = Vec::new();

      v.push($x);

      v
    }
  };

  [$($x:expr),+] => (
    {
      let mut v = Vec::new();

      $(
        v.push($x);
      )+

      v
    }
  )
}

#[cfg(test)]
mod test {
  use tracing::info;

  #[test]
  fn test1() {
    crate::init_tracing();

    let empty: Vec<i32> = my_vec![];

    info!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec!(make an empty vec);

    info!("{:?}", also_empty);

    let three_numbers: Vec<i32> = my_vec!(1, 2, 3);

    info!("{:?}", three_numbers);
  }
}
