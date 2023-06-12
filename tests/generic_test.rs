use std::marker::PhantomData;

use astmaker::{ast, model};


ast!{
  location = ();

  pub node Program = {
    instructions: Vec<Node<Instruction>>,
  }

  pub node Instruction =
    | Word -> Node<Word>
    | Number -> Node<Number>
    ;

  pub node Word = {}
  pub node Number = {}
}

pub struct GenericModel<T> {
  n_words: usize,
  n_numbers: usize,
  marker: PhantomData<T>,
}

model!{
  impl<T> GenericModel -> () {
    where Program => {
      for child in node.data.instructions.iter_mut() {
        context.visit(child);
      }
    },
    where Instruction => {
      match node.data.as_mut() {
        Instruction::Word(child) => context.visit(child),
        Instruction::Number(child) => context.visit(child),
      }
    },
    where Word => {
      context.n_words += 1;
    },
    where Number => {
      context.n_numbers += 1;
    },
  }
}


#[test]
fn eval() {
  let mut tree = Node::new((), Program {
    instructions: vec![
      Node::new((), Instruction::Word(
        Node::new((), Word {})
      )),
      Node::new((), Instruction::Number(
        Node::new((), Number {})
      )),
      Node::new((), Instruction::Word(
        Node::new((), Word {})
      )),
    ],
  });

  let mut model: GenericModel<u8> = GenericModel {
    n_words: 0,
    n_numbers: 0,
    marker: PhantomData,
  };
  model.visit(&mut tree);
  assert_eq!(model.n_words, 2);
  assert_eq!(model.n_numbers, 1);
}
