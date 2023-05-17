use proc_macro::TokenStream;

mod parser;
mod codegen;

#[proc_macro]
pub fn ast(input: TokenStream) -> TokenStream {
  codegen::ast::generate(input)
}

#[proc_macro]
pub fn model(input: TokenStream) -> TokenStream {
  codegen::model::generate(input)
}
