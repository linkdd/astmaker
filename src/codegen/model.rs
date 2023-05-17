use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parser::model::*;

pub fn generate(input: TokenStream) -> TokenStream {
  let Model {
    context,
    output,
    clauses,
  } = parse_macro_input!(input as Model);

  let typedefs = quote!{
    pub trait Visitable<T: NodeAttributes> {
      fn visit(context: &mut #context, node: &mut Node<T>) -> #output;
    }
  };

  let clause_defs = clauses
    .iter()
    .map(|Clause { pattern, body }| quote!{
      impl Visitable<#pattern> for #pattern {
        fn visit(context: &mut #context, node: &mut Node<#pattern>) -> #output {
          #body
        }
      }
    });

  TokenStream::from(quote!{
    #typedefs
    #(#clause_defs)*

    impl #context {
      pub fn visit<T: NodeAttributes + Visitable<T>>(&mut self, node: &mut Node<T>) -> #output {
        T::visit(self, node)
      }
    }
  })
}
