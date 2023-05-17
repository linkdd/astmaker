use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parser::ast::*;

pub fn generate(input: TokenStream) -> TokenStream {
  let AbstractSyntaxTree {
    location,
    nodes,
  } = parse_macro_input!(input as AbstractSyntaxTree);

  let location_type = location.datatype;

  let typedefs = quote!{
    pub trait NodeAttributes {
      type Attributes;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Node<T: NodeAttributes> {
      pub location: #location_type,
      pub attrs: Option<T::Attributes>,
      pub data: Box<T>,
    }

    impl<T: NodeAttributes> Node<T> {
      pub fn new(location: #location_type, data: T) -> Self {
        Self { location, attrs: None, data: Box::new(data) }
      }
    }
  };

  let mut node_defs = vec![];

  for node in nodes {
    let Node {
      visibility,
      name,
      attrs,
      data,
    } = node;

    let attrs_type = match attrs {
      None => quote!{()},
      Some(NodeAttributes { datatype }) => quote!{ #datatype }
    };

    let node_def = match data {
      NodeData::Struct(data) => {
        let fields = data.members
          .iter()
          .map(|NodeDataStructField { name, datatype }| quote!{
            #name : #datatype
          });

        quote!{
          #[derive(Debug, Clone, PartialEq)]
          #visibility struct #name {
            #(#fields),*
          }

          impl NodeAttributes for #name {
            type Attributes = #attrs_type;
          }
        }
      },
      NodeData::Enum(data) => {
        let variants = data.variants
          .iter()
          .map(|NodeDataEnumVariant { name, datatype }| quote!{
            #name ( #datatype )
          });

        quote!{
          #[derive(Debug, Clone, PartialEq)]
          #visibility enum #name {
            #(#variants),*
          }

          impl NodeAttributes for #name {
            type Attributes = #attrs_type;
          }
        }
      },
    };

    node_defs.push(node_def);
  }

  TokenStream::from(quote!{
    #typedefs

    #(#node_defs)*
  })
}
