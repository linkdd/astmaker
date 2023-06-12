# astmaker

[![Crates.io](https://img.shields.io/crates/v/astmaker?style=flat-square)](https://crates.io/crates/astmaker)
[![Crates.io](https://img.shields.io/crates/l/astmaker?style=flat-square)](https://crates.io/crates/astmaker)
[![Crates.io](https://img.shields.io/crates/d/astmaker?style=flat-square)](https://crates.io/crates/astmaker)
[![docs.rs](https://img.shields.io/docsrs/astmaker?style=flat-square)](https://docs.rs/astmaker)

Build Abstract Syntax Trees and tree-walking models quickly in Rust.

## Example

This example creates an AST for simple math expressions, and an interpreter to
evaluate the result:

```rust
use astmaker::{ast, model};

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
  Add,
  Sub,
}

ast!{
  location = ();

  pub node Expression =
    | BinOp -> Node<BinaryOperation>
    | UnOp -> Node<UnaryOperation>
    | Num -> Node<Number>
    ;

  pub node BinaryOperation = {
    lhs: Node<Expression>,
    op: BinOp,
    rhs: Node<Expression>,
  }

  pub node UnaryOperation = {
    op: UnOp,
    expr: Node<Expression>,
  }

  pub node Number = {
    value: f64,
  }
}

pub struct Interpreter;

model!{
  impl Interpreter -> f64 {
    where Expression => {
      match node.data.as_mut() {
        Expression::BinOp(child_node) => context.visit(child_node),
        Expression::UnOp(child_node) => context.visit(child_node),
        Expression::Num(child_node) => context.visit(child_node),
      }
    },
    where BinaryOperation => {
      let lhs = context.visit(&mut node.data.lhs);
      let rhs = context.visit(&mut node.data.rhs);

      match node.data.op {
        BinOp::Add => lhs + rhs,
        BinOp::Sub => lhs - rhs,
        BinOp::Mul => lhs * rhs,
        BinOp::Div => lhs / rhs,
      }
    },
    where UnaryOperation => {
      let val = context.visit(&mut node.data.expr);

      match node.data.op {
        UnOp::Add => val,
        UnOp::Sub => -val,
      }
    },
    where Number => node.data.value,
  }
}

#[test]
fn eval() {
  let mut tree = Node::new((), Expression::BinOp(
    Node::new((), BinaryOperation {
      lhs: Node::new((), Expression::Num(
        Node::new((), Number { value: 1.0 })
      )),
      op: BinOp::Add,
      rhs: Node::new((), Expression::Num(
        Node::new((), Number { value: 2.0 })
      ))
    })
  ));

  let mut interpreter = Interpreter;

  assert_eq!(interpreter.visit(&mut tree), 3.0);
}
```

## Roadmap

 - [x] :memo: Documentation
 - [x] :sparkles: Generics support
 - [x] :sparkles: Lifetimes support
 - [ ] :construction_worker: Github workflows to build, test & publish crate

## License

This project is released under the terms of the [MIT License](./LICENSE.txt).
