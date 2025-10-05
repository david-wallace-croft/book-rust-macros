#![allow(dead_code)]

struct Age {
  value: i32,
}

struct FirstName {
  value: String,
}

impl FirstName {
  pub fn new(name: &str) -> Result<FirstName, String> {
    if name.len() < 2 {
      Err("Name should be at least two characters".to_string())
    } else {
      Ok(FirstName {
        value: name.to_string(),
      })
    }
  }
}
struct LastName {
  value: String,
}

impl LastName {
  pub fn new(name: &str) -> Result<LastName, String> {
    if name.len() < 2 {
      Err("Name should be at least two characters".to_string())
    } else {
      Ok(LastName {
        value: name.to_string(),
      })
    }
  }
}

struct Pay {
  value: i32,
}

fn calculate_raise(
  first_name: FirstName,
  _last_name: LastName,
  _age: Age,
  current_pay: Pay,
) -> Pay {
  if first_name.get_value() == "David" {
    return Pay {
      value: current_pay.get_value() + 1,
    };
  }

  current_pay
}

macro_rules! generate_from {
  ($struct_type:ident) => {
    generate_from!($struct_type, String);
  };

  ($struct_type:ident, $return_type:ty) => {
    impl From<$struct_type> for $return_type {
      fn from(f: $struct_type) -> Self {
        f.value
      }
    }
  };
}

macro_rules! generate_get_value {
  ($struct_type:ident) => {
    generate_get_value!($struct_type, String);
  };

  ($struct_type:ident, $return_type:ty) => {
    impl $struct_type {
      pub fn get_value(&self) -> &$return_type {
        &self.value
      }
    }
  };
}

macro_rules! generate_newtypes {
  ($struct_type:ident) => {
    generate_from!($struct_type, String);

    generate_get_value!($struct_type, String);
  };

  ($struct_type:ident, $return_type:ty) => {
    generate_from!($struct_type, $return_type);

    generate_get_value!($struct_type, $return_type);
  };
}

generate_newtypes!(FirstName);
generate_newtypes!(LastName);
generate_newtypes!(Age, i32);
generate_newtypes!(Pay, i32);

#[cfg(test)]
mod test {
  use super::*;
  // use tracing::info;

  #[test]
  fn test1() -> Result<(), String> {
    // crate::init_tracing();

    let first_name: FirstName = FirstName::new("David")?;

    let last_name: LastName = LastName::new("Croft")?;

    let age: Age = Age {
      value: 0,
    };

    let current_pay: Pay = Pay {
      value: 0,
    };

    let pay: Pay = calculate_raise(first_name, last_name, age, current_pay);

    assert_eq!(1, pay.value);

    Ok(())
  }

  #[test]
  fn test2() -> Result<(), String> {
    // crate::init_tracing();

    let first_name: FirstName = FirstName::new("Divad")?;

    let last_name: LastName = LastName::new("Croft")?;

    let age: Age = Age {
      value: 0,
    };

    let current_pay: Pay = Pay {
      value: 0,
    };

    let pay: Pay = calculate_raise(first_name, last_name, age, current_pay);

    assert_eq!(0, pay.value);

    Ok(())
  }

  #[test]
  fn test3() -> Result<(), String> {
    // crate::init_tracing();

    let first_name: FirstName = FirstName::new("David")?;

    let first_name_string: String = String::from(first_name);

    assert_eq!(first_name_string, "David");

    let age: Age = Age {
      value: 1,
    };

    let age_value: i32 = i32::from(age);

    assert_eq!(1, age_value);

    Ok(())
  }
}
