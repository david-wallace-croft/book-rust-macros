#![expect(dead_code)]

use ::syn::{
  Ident, LitInt, Token,
  parse::{Parse, ParseStream},
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
pub struct IacInput {
  bucket: Option<Bucket>,
  lambda: Option<Lambda>,
}

impl Parse for IacInput {
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

    Ok(IacInput {
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
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let lambda_token: kw::lambda = input
      .parse::<kw::lambda>()
      .expect("we just checked for this token");

    let lambda_name: String =
      input.parse().map(|v: Ident| v.to_string()).map_err(|_| {
        syn::Error::new(lambda_token.span(), "lambda needs a name")
      })?;

    let mut lambda_memory: Option<u16> = None;

    let mut lambda_timeout: Option<u16> = None;

    while !input.is_empty() && !input.peek(kw::bucket) {
      if input.peek(kw::mem) {
        let _ = input
          .parse::<kw::mem>()
          .expect("we just checked for this token");

        lambda_memory = Some(input.parse().map(|v: LitInt| {
          v.to_string().parse().map_err(|_| {
            syn::Error::new(v.span(), "memory needs positive value <= 1024")
          })
        })??);
      } else if input.peek(kw::time) {
        let _ = input
          .parse::<kw::time>()
          .expect("we just checked for this token");

        lambda_timeout = Some(input.parse().map(|v: LitInt| {
          v.to_string().parse().map_err(|_| {
            syn::Error::new(v.span(), "timeout needs positive value <= 900")
          })
        })??);
      } else {
        Err(syn::Error::new(
          input.span(),
          "unknown property passed to lambda",
        ))?
      }
    }

    Ok(Lambda {
      memory: lambda_memory,
      name: lambda_name,
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
