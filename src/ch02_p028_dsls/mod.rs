#![allow(dead_code)]
#![allow(unused_macros)]

use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
struct Account {
  money: u32,
}

impl Account {
  fn add(
    &mut self,
    money: u32,
  ) {
    self.money = self.money.add(money)
  }

  fn subtract(
    &mut self,
    money: u32,
  ) {
    self.money = self.money.sub(money)
  }
}

macro_rules! exchange {
  (Give $amount:literal to $name:ident) => {
    $name.add($amount)
  };

  (Take $amount:literal from $name:ident) => {
    $name.subtract($amount)
  };

  (Give $amount:literal from $giver:ident to $receiver:ident) => {
    $giver.subtract($amount);

    $receiver.add($amount);
  };
}

macro_rules! give_money_to_the_poor {
  (Give 0) => {
    info!("Cheapskate");
  };

  (Give $example:literal) => {
    info!("How generous");
  };
}

#[cfg(test)]
mod test {
  use super::*;
  use tracing::info;

  #[test]
  fn test1() {
    crate::init_tracing();

    let mut the_poor = Account {
      money: 0,
    };

    let mut the_rich = Account {
      money: 200,
    };

    exchange!(Give 20 to the_poor);

    exchange!(Take 10 from the_rich);

    exchange!(Give 30 from the_rich to the_poor);

    info!("Poor: {the_poor:?}, rich: {the_rich:?}");
  }
  #[test]
  fn test2() {
    crate::init_tracing();

    give_money_to_the_poor!(Give 0);

    give_money_to_the_poor!(Give 1);
  }
}
