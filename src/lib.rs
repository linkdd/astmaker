//! # Introduction
//!
//! `astmaker` is a DSL for programming language designers to build Abstract
//! Syntax Trees and tree-walking models quickly.
//!
//! # Features
//!
//! AST definition:
//!
//!  - custom location type
//!  - structural nodes (`struct`) and variant nodes (`enum`)
//!  - custom node attributes type
//!
//! Model definition:
//!
//!  - visitor pattern
//!  - support for generics and lifetimes
//!
//! # Architecture
//!
//! When creating an AST, this crate will define the following types and traits:
//!
//! ```rust
//! pub trait NodeAttributes {
//!   type Attributes;
//! }
//!
//! #[derive(Debug, Clone, PartialEq)]
//! pub struct Node<T: NodeAttributes> {
//!   pub location: LocationType,
//!   pub data: Box<T>,
//!   pub attrs: Option<T::NodeAttributes>,
//! }
//! ```
//!
//! When creating a model, this crate will define the following types:
//!
//! ```rust
//! pub trait Visitor: Sized {
//!   fn visit<T: NodeAttributes + Visitable<Self, T>>(
//!     &mut self,
//!     node: &mut Node<T>,
//!   ) -> OutputType;
//! }
//!
//! pub trait Visitable<C: Visitor, T: NodeAttributes> {
//!   fn visit(context: &mut C, node: &mut Node<T>) -> OutputType;
//! }
//! ```
//!
//! # Basic usage
//!
//! This crates provide 2 macros:
//!
//!  - `ast!`: to define the AST
//!  - `model!`: to implement the tree-walking model
//!
//! Each macro provide a custom DSL.
//!
//! ## Defining Abstract Syntax Tress
//!
//! ```rust
//! use astmaker::{ast, model};
//!
//! ast!{
//!   location = (usize, usize);
//!
//!   pub node VariantNode =
//!     | A -> Node<StructuralNodeA>
//!     | B -> Node<StructuralNodeB>
//!     ;
//!
//!   pub node StructuralNodeA = {
//!     data: u8,
//!   }
//!
//!   pub node StructuralNodeB = {
//!     data: u16,
//!   }
//!
//!   pub node NodeWithAttributes where attrs: String = {
//!     data: u32,
//!   }
//! }
//! ```
//!
//! When not specified, the default attributes type is the unit type `()`.
//!
//! The generated code will contain the `struct`s and `enum`s as well as their
//! implementation of the `NodeAttributes` trait.
//!
//! Every generated type implements the traits `Debug`, `Clone` and `PartialEq`.
//!
//! ## Defining tree-walking models
//!
//! ```rust
//! pub struct Model;
//!
//! model!{
//!   impl Model -> Result<(), ()> {
//!     where VariantNode => {
//!       match node.data.as_mut() {
//!         VariantNode::A(child) => context.visit(child)?,
//!         VariantNode::B(child) => context.visit(child)?,
//!       }
//!
//!       Ok(())
//!     },
//!     where StructuralNodeA => {
//!       Ok(())
//!     },
//!     where StructuralNodeB => {
//!       Ok(())
//!     },
//!   }
//! }
//! ```
//!
//! The `impl for Type` part will implement the `Visitor` trait for the supplied
//! type. Each `where` clause will implement the `Visitable` trait for the node
//! type.
//!
//! Generics and lifetimes are also supported:
//!
//! ```rust
//! pub struct Model<'a, T> {
//!   data: &'a T,
//! }
//!
//! model!{
//!   impl<'a, T> Model -> Result<(), ()> {
//!     // ...
//!   }
//! }
//! ```

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
