#![allow(unused_macros)]

macro_rules! my_vec_expr {
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

macro_rules! my_vec_literal {
  () => [
    Vec::new()
  ];

  (make an empty vec) => (
    Vec::new()
  );

  {$x:literal} => {
    {
      let mut v = Vec::new();

      v.push($x);

      v
    }
  };

  [$($x:literal),+] => (
    {
      let mut v = Vec::new();

      $(
        v.push($x);
      )+

      v
    }
  )
}

macro_rules! my_vec_tt {
  () => [
    Vec::new()
  ];

  (make an empty vec) => (
    Vec::new()
  );

  {$x:tt} => {
    {
      let mut v = Vec::new();

      v.push($x);

      v
    }
  };

  [$($x:tt),+] => (
    {
      let mut v = Vec::new();

      $(
        v.push($x);
      )+

      v
    }
  )
}

// macro_rules! my_vec_ident {
//   () => [
//     Vec::new()
//   ];

//   (make an empty vec) => (
//     Vec::new()
//   );

//   {$x:ident} => {
//     {
//       let mut v = Vec::new();

//       v.push($x);

//       v
//     }
//   };

//   [$($x:ident),+] => (
//     {
//       let mut v = Vec::new();

//       $(
//         v.push($x);
//       )+

//       v
//     }
//   )
// }

// macro_rules! my_vec_ty {
//   () => [
//     Vec::new()
//   ];

//   (make an empty vec) => (
//     Vec::new()
//   );

//   {$x:ty} => {
//     {
//       let mut v = Vec::new();

//       v.push($x);

//       v
//     }
//   };

//   [$($x:ty),+] => (
//     {
//       let mut v = Vec::new();

//       $(
//         v.push($x);
//       )+

//       v
//     }
//   )
// }

#[cfg(test)]
mod test {
  use tracing::info;

  #[test]
  fn test1() {
    crate::init_tracing();

    let empty: Vec<i32> = my_vec_expr![];

    info!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec_expr!(make an empty vec);

    info!("{:?}", also_empty);

    let three_numbers: Vec<i32> = my_vec_expr!(1, 2, 3);

    info!("{:?}", three_numbers);
  }

  #[test]
  fn test2() {
    crate::init_tracing();

    let empty: Vec<i32> = my_vec_literal![];

    info!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec_literal!(make an empty vec);

    info!("{:?}", also_empty);

    let three_numbers: Vec<i32> = my_vec_literal!(1, 2, 3);

    info!("{:?}", three_numbers);
  }

  #[test]
  fn test3() {
    crate::init_tracing();

    let empty: Vec<i32> = my_vec_tt![];

    info!("{:?}", empty);

    let also_empty: Vec<i32> = my_vec_tt!(make an empty vec);

    info!("{:?}", also_empty);

    let three_numbers: Vec<i32> = my_vec_tt!(1, 2, 3);

    info!("{:?}", three_numbers);
  }

  // #[test]
  // fn test4() {
  //   crate::init_tracing();

  //   let empty: Vec<i32> = my_vec_ident![];

  //   info!("{:?}", empty);

  //   let also_empty: Vec<i32> = my_vec_ident!(make an empty vec);

  //   info!("{:?}", also_empty);

  //   fn fn0() -> usize {
  //     return 0;
  //   }

  //   fn fn1() -> usize {
  //     return 1;
  //   }

  //   fn fn2() -> usize {
  //     return 2;
  //   }

  //   let three_functions = my_vec_ident!(fn0, fn1, fn2);

  //   assert_eq!(0, three_functions[0]());

  //   assert_eq!(1, three_functions[1]());

  //   assert_eq!(2, three_functions[2]());
  // }

  // #[test]
  // fn test5() {
  //   crate::init_tracing();

  //   let empty: Vec<i32> = my_vec_ty![];

  //   info!("{:?}", empty);

  //   let also_empty: Vec<i32> = my_vec_ty!(make an empty vec);

  //   info!("{:?}", also_empty);

  //   let three_numbers = my_vec_ty!(i8, i16, i32);

  //   info!("{:?}", three_numbers);
  // }
}
