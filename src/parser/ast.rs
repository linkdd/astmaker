use syn::parse::{Parse, ParseStream, Result};
use syn::{Error, Token, Visibility, Ident, Type, braced};

pub struct AbstractSyntaxTree {
  pub location: LocationInfo,
  pub nodes: Vec<Node>,
}

pub struct LocationInfo {
  pub datatype: Type,
}

pub struct Node {
  pub visibility: Visibility,
  pub name: Ident,
  pub attrs: Option<NodeAttributes>,
  pub data: NodeData,
}

pub struct NodeAttributes {
  pub datatype: Type,
}

pub enum NodeData {
  Struct(NodeDataStruct),
  Enum(NodeDataEnum),
}

pub struct NodeDataStruct {
  pub members: Vec<NodeDataStructField>,
}

pub struct NodeDataStructField {
  pub name: Ident,
  pub datatype: Type,
}

pub struct NodeDataEnum {
  pub variants: Vec<NodeDataEnumVariant>,
}

pub struct NodeDataEnumVariant {
  pub name: Ident,
  pub datatype: Type,
}

impl Parse for AbstractSyntaxTree {
  fn parse(input: ParseStream) -> Result<Self> {
    let location: LocationInfo = input.parse()?;
    let mut nodes = vec![];

    while !input.is_empty() {
      let node: Node = input.parse()?;
      nodes.push(node);
    }

    Ok(Self { location, nodes })
  }
}

impl Parse for LocationInfo {
  fn parse(input: ParseStream) -> Result<Self> {
    let location_token: Ident = input.parse()?;
    if location_token != "location" {
      return Err(Error::new(location_token.span(), "expected `location`"));
    }

    input.parse::<Token![=]>()?;

    let datatype: Type = input.parse()?;

    input.parse::<Token![;]>()?;

    Ok(Self { datatype })
  }
}

impl Parse for Node {
  fn parse(input: ParseStream) -> Result<Self> {
    let visibility: Visibility = input.parse()?;

    let node_token: Ident = input.parse()?;
    if node_token != "node" {
      return Err(Error::new(node_token.span(), "expected `node`"));
    }

    let name: Ident = input.parse()?;

    input.lookahead1();

    let attrs = if input.peek(Token![where]) {
      Some(input.parse()?)
    }
    else {
      None
    };

    input.parse::<Token![=]>()?;
    let data: NodeData = input.parse()?;

    Ok(Self { visibility, name, attrs, data })
  }
}

impl Parse for NodeAttributes {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![where]>()?;

    let attrs_token: Ident = input.parse()?;
    if attrs_token != "attrs" {
      return Err(Error::new(attrs_token.span(), "expected `attrs`"));
    }

    input.parse::<Token![:]>()?;

    let datatype: Type = input.parse()?;

    Ok(Self { datatype })
  }
}

impl Parse for NodeData {
  fn parse(input: ParseStream) -> Result<Self> {
    input.lookahead1();

    if input.peek(Token![|]) {
      input.parse().map(NodeData::Enum)
    }
    else {
      input.parse().map(NodeData::Struct)
    }
  }
}

impl Parse for NodeDataStruct {
  fn parse(input: ParseStream) -> Result<Self> {
    let content;
    braced!(content in input);

    let members = content
      .parse_terminated(NodeDataStructField::parse, Token![,])?
      .into_iter()
      .collect();

    Ok(Self { members })
  }
}

impl Parse for NodeDataStructField {
  fn parse(input: ParseStream) -> Result<Self> {
    let name: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let datatype: Type = input.parse()?;

    Ok(Self { name, datatype })
  }
}

impl Parse for NodeDataEnum {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut variants = vec![];

    loop {
      input.lookahead1();
      if input.peek(Token![;]) {
        break;
      }

      input.parse::<Token![|]>()?;
      let variant: NodeDataEnumVariant = input.parse()?;
      variants.push(variant);
    }

    input.parse::<Token![;]>()?;
    Ok(Self { variants })
  }
}

impl Parse for NodeDataEnumVariant {
  fn parse(input: ParseStream) -> Result<Self> {
    let name: Ident = input.parse()?;
    input.parse::<Token![->]>()?;
    let datatype: Type = input.parse()?;
    Ok(Self { name, datatype })
  }
}
