use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Type, Expr, braced};


pub struct Model {
  pub context: Type,
  pub output: Type,
  pub clauses: Vec<Clause>,
}

pub struct Clause {
  pub pattern: Type,
  pub body: Expr,
}

impl Parse for Model {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![for]>()?;

    let context: Type = input.parse()?;

    input.parse::<Token![->]>()?;

    let output: Type = input.parse()?;

    let content;
    braced!(content in input);

    let clauses = content
      .parse_terminated(Clause::parse, Token![,])?
      .into_iter()
      .collect();

    Ok(Self { context, output, clauses })
  }
}

impl Parse for Clause {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![where]>()?;

    let pattern: Type = input.parse()?;

    input.parse::<Token![=>]>()?;

    let body: Expr = input.parse()?;

    Ok(Self { pattern, body })
  }
}
