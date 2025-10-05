#![allow(dead_code)]
#![allow(unused_macros)]

use std::ops::Add;
use std::ops::Sub;

enum CurrencyType {
  Dollars,
  Euros,
}

#[derive(Debug)]
struct Account {
  dollars: u32,
}

impl Account {
  fn add(
    &mut self,
    currency_type: CurrencyType,
    money: u32,
  ) {
    let exchange_rate: u32 = match currency_type {
      CurrencyType::Dollars => 1,
      CurrencyType::Euros => 2,
    };

    let amount: u32 = exchange_rate * money;

    self.dollars = self.dollars.add(amount);
  }

  fn subtract(
    &mut self,
    currency_type: CurrencyType,
    money: u32,
  ) {
    let exchange_rate: u32 = match currency_type {
      CurrencyType::Dollars => 1,
      CurrencyType::Euros => 2,
    };

    let amount: u32 = exchange_rate * money;

    self.dollars = self.dollars.sub(amount)
  }
}

macro_rules! exchange {
  (Give $amount:literal to $name:ident) => {
    $name.add(CurrencyType::Dollars, $amount)
  };

  (Give $amount:literal in Dollars to $name:ident) => {
    $name.add(CurrencyType::Dollars, $amount)
  };

  (Give $amount:literal in Euros to $name:ident) => {
    $name.add(CurrencyType::Euros, $amount)
  };

  (Take $amount:literal from $name:ident) => {
    $name.subtract(CurrencyType::Dollars, $amount)
  };

  (Take $amount:literal in Dollars from $name:ident) => {
    $name.subtract(CurrencyType::Dollars, $amount)
  };

  (Take $amount:literal in Euros from $name:ident) => {
    $name.subtract(CurrencyType::Euros, $amount)
  };

  (Give $amount:literal from $giver:ident to $receiver:ident) => {
    $giver.subtract(CurrencyType::Dollars, $amount);

    $receiver.add(CurrencyType::Dollars, $amount);
  };

  (Give $amount:literal in Dollars from $giver:ident to $receiver:ident) => {
    $giver.subtract(CurrencyType::Dollars, $amount);

    $receiver.add(CurrencyType::Dollars, $amount);
  };

  (Give $amount:literal in Euros from $giver:ident to $receiver:ident) => {
    $giver.subtract(CurrencyType::Euros, $amount);

    $receiver.add(CurrencyType::Euros, $amount);
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

    let mut the_poor: Account = Account {
      dollars: 0,
    };

    let mut the_rich: Account = Account {
      dollars: 1_000,
    };

    info!("Poor: {the_poor:?}, rich: {the_rich:?}");

    exchange!(Give 1 in Dollars to the_poor);

    exchange!(Give 2 in Euros to the_poor);

    exchange!(Take 10 in Dollars from the_rich);

    exchange!(Take 20 in Euros from the_rich);

    exchange!(Give 100 in Dollars from the_rich to the_poor);

    exchange!(Give 200 in Euros from the_rich to the_poor);

    info!("Poor: {the_poor:?}, rich: {the_rich:?}");

    assert_eq!(505, the_poor.dollars);

    assert_eq!(450, the_rich.dollars);
  }
}
