#![expect(dead_code)]

use ::proc_macro2::Span;
use ::syn::{
  Ident, LitInt, Token, parenthesized,
  parse::{Lookahead1, Parse, ParseBuffer, ParseStream},
  punctuated::Punctuated,
  spanned::Spanned,
  token::Comma,
};

#[derive(Debug)]
pub struct Bucket {
  has_event: bool,
  name: String,
}

impl Parse for Bucket {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let bucket_token: kw::bucket = input
      .parse::<kw::bucket>()
      .expect("we just checked for this token");

    let bucket_name: String =
      input.parse().map(|v: Ident| v.to_string()).map_err(|_| {
        syn::Error::new(bucket_token.span(), "bucket needs a name")
      })?;

    let event_needed: bool =
      if !input.peek(kw::lambda) && input.peek(Token!(=>)) {
        let _ = input.parse::<Token!(=>)>().unwrap();

        true
      } else {
        false
      };

    Ok(Bucket {
      has_event: event_needed,
      name: bucket_name,
    })
  }
}

#[derive(Debug)]
struct KeyValue {
  key: String,
  value: String,
}

impl Parse for KeyValue {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let key: String =
      input.parse().map(|v: Ident| v.to_string()).map_err(|_| {
        syn::Error::new(
          input.span(),
          "should have property keys within parentheses",
        )
      })?;

    let _: Token!(=) = input.parse().map_err(|_| {
      syn::Error::new(
        input.span(),
        "prop name and value should be separated by =",
      )
    })?;

    let value: String = if key == "name" {
      input.parse().map(|v: Ident| v.to_string()).map_err(|_| {
        syn::Error::new(input.span(), "Name property needs a value")
      })
    } else if key == "mem" || key == "time" {
      input.parse().map(|v: LitInt| v.to_string()).map_err(|_| {
        syn::Error::new(input.span(), "memory and time need a positive value")
      })
    } else {
      Err(syn::Error::new(
        input.span(),
        format!("unknown property for lambda: {key}"),
      ))
    }?;

    Ok(KeyValue {
      key,
      value,
    })
  }
}

#[derive(Debug)]
pub struct IacInputUsing {
  bucket: Option<Bucket>,
  lambda: Option<Lambda>,
}

impl Parse for IacInputUsing {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut bucket: Option<Bucket> = None;

    let mut lambda: Option<Lambda> = None;

    loop {
      if input.peek(kw::bucket) {
        bucket = Some(input.parse()?);
      } else if input.peek(kw::lambda) {
        lambda = Some(input.parse()?);
      } else if !input.is_empty() {
        return Err(syn::Error::new(
          input.lookahead1().error().span(),
          "only 'bucket' and 'lambda' resources are supported",
        ));
      } else {
        break;
      }
    }

    if bucket
      .as_ref()
      .map(|v: &Bucket| v.has_event)
      .unwrap_or(false)
      && lambda.is_none()
    {
      return Err(syn::Error::new(
        input.span(),
        "a lambda is required for an event ('=>')",
      ));
    }

    Ok(IacInputUsing {
      bucket,
      lambda,
    })
  }
}

#[derive(Debug)]
pub struct Lambda {
  memory: Option<u16>,
  name: String,
  time: Option<u16>,
}

impl Lambda {
  fn builder(input_span: Span) -> LambdaBuilder {
    LambdaBuilder {
      input_span,
      memory: None,
      name: None,
      time: None,
    }
  }
}

impl Parse for Lambda {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    let _ = input
      .parse::<kw::lambda>()
      .expect("we just checked for this token");

    let content: ParseBuffer<'_>;

    parenthesized!(content in input);

    let key_values: Punctuated<LambdaProperty, Comma> =
      Punctuated::<LambdaProperty, Token!(,)>::parse_terminated(&content)?;

    let builder = key_values.into_iter().fold(
      Lambda::builder(content.span()),
      |accumulator: LambdaBuilder, current| match current {
        LambdaProperty::Name(val) => accumulator.name(val),
        LambdaProperty::Memory(val) => accumulator.memory(val),
        LambdaProperty::Time(val) => accumulator.time(val),
      },
    );

    builder.build()
  }
}

struct LambdaBuilder {
  input_span: Span,
  memory: Option<u16>,
  name: Option<String>,
  time: Option<u16>,
}

impl LambdaBuilder {
  fn build(self) -> Result<Lambda, syn::Error> {
    let name: String = self.name.ok_or(syn::Error::new(
      self.input_span,
      "name is required for lambda",
    ))?;

    Ok(Lambda {
      memory: self.memory,
      name,
      time: self.time,
    })
  }

  fn memory(
    mut self,
    memory: u16,
  ) -> Self {
    self.memory = Some(memory);

    self
  }

  fn name(
    mut self,
    name: String,
  ) -> Self {
    self.name = Some(name);

    self
  }

  fn time(
    mut self,
    time: u16,
  ) -> Self {
    self.time = Some(time);

    self
  }
}

#[derive(Debug)]
enum LambdaProperty {
  Memory(u16),
  Name(String),
  Time(u16),
}

impl LambdaProperty {
  fn parse_number<T>(
    input: ParseStream,
    error_message: &str,
  ) -> Result<u16, syn::Error>
  where
    T: Parse,
  {
    let _ = input.parse::<T>().expect("we just checked for this token");

    let _: Token!(=) = input.parse().map_err(|_| {
      syn::Error::new(
        input.span(),
        "prop name anad value should be separated by =",
      )
    })?;

    let value: u16 = input.parse().map(|v: LitInt| {
      v.to_string()
        .parse()
        .map_err(|_| syn::Error::new(v.span(), error_message))
    })??;

    Ok(value)
  }
}

impl Parse for LambdaProperty {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let lookahead: Lookahead1<'_> = input.lookahead1();

    if lookahead.peek(kw::mem) {
      let value: u16 = LambdaProperty::parse_number::<kw::mem>(
        input,
        "memory needs a postive value <= 10240",
      )?;

      Ok(LambdaProperty::Memory(value))
    } else if lookahead.peek(kw::name) {
      let _ = input
        .parse::<kw::name>()
        .expect("we just checked for this token");

      let _: Token!(=) = input.parse().map_err(|_| {
        syn::Error::new(
          input.span(),
          "prop name and value should be separated by =",
        )
      })?;

      let value: String =
        input.parse().map(|v: Ident| v.to_string()).map_err(|_| {
          syn::Error::new(input.span(), "name property needs a value")
        })?;

      Ok(LambdaProperty::Name(value))
    } else if lookahead.peek(kw::time) {
      let value: u16 = LambdaProperty::parse_number::<kw::time>(
        input,
        "memory needs a postive value <= 900",
      )?;

      Ok(LambdaProperty::Time(value))
    } else {
      Err(syn::Error::new(input.span(), "unknown property for lambda"))
    }
  }
}

mod kw {
  use ::syn::custom_keyword;

  custom_keyword!(bucket);

  custom_keyword!(lambda);

  custom_keyword!(mem);

  custom_keyword!(name);

  custom_keyword!(time);
}
