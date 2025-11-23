#![expect(dead_code)]

use ::syn::{
  Ident, LitInt, Token, parenthesized,
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  spanned::Spanned,
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
pub struct IacInputStruct {
  bucket: Option<Bucket>,
  lambda: Option<Lambda>,
}

impl Parse for IacInputStruct {
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

    Ok(IacInputStruct {
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

impl Parse for Lambda {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    let _ = input
      .parse::<kw::lambda>()
      .expect("we just checked for this token");

    let mut lambda_name = None;

    let mut lambda_memory: Option<u16> = None;

    let mut lambda_timeout: Option<u16> = None;

    let content;

    parenthesized!(content in input);

    let key_values =
      Punctuated::<KeyValue, Token!(,)>::parse_terminated(&content)?;

    key_values.into_iter().for_each(|key_value: KeyValue| {
      if key_value.key == "name" {
        lambda_name = Some(key_value.value);
      } else if key_value.key == "mem" {
        lambda_memory = Some(key_value.value.parse().unwrap());
      } else if key_value.key == "time" {
        lambda_timeout = Some(key_value.value.parse().unwrap())
      }
    });

    Ok(Lambda {
      memory: lambda_memory,
      name: lambda_name
        .ok_or(syn::Error::new(input.span(), "lambda needs a name"))?,
      time: lambda_timeout,
    })
  }
}

mod kw {
  use ::syn::custom_keyword;

  custom_keyword!(bucket);

  custom_keyword!(lambda);

  custom_keyword!(mem);

  custom_keyword!(time);
}
