use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parser::model::*;

pub fn generate(input: TokenStream) -> TokenStream {
  let Model {
    context,
    generics,
    output,
    clauses,
  } = parse_macro_input!(input as Model);

  let typedefs = quote!{
    pub trait Visitor: Sized {
      fn visit<T: NodeAttributes + Visitable<Self, T>>(&mut self, node: &mut Node<T>) -> #output;
    }

    pub trait Visitable<C: Visitor, T: NodeAttributes> {
      fn visit(context: &mut C, node: &mut Node<T>) -> #output;
    }
  };

  let (
    impl_generics,
    ty_generics,
    where_clause,
  ) = generics.split_for_impl();

  let clause_defs = clauses
    .iter()
    .map(|Clause { pattern, body }| quote!{
      impl #impl_generics Visitable<#context #ty_generics, #pattern> for #pattern #where_clause {
        fn visit(context: &mut #context #ty_generics, node: &mut Node<#pattern>) -> #output {
          #body
        }
      }
    });

  TokenStream::from(quote!{
    #typedefs
    #(#clause_defs)*

    impl #impl_generics Visitor for #context #ty_generics #where_clause {
      fn visit<T__: NodeAttributes + Visitable<Self, T__>>(&mut self, node: &mut Node<T__>) -> #output {
        T__::visit(self, node)
      }
    }
  })
}
