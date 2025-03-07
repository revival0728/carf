pub mod token;

use token::Token;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AstKind {
  Root,
  Fun,
  Let,
  Stmt,
  Expr,
  Call,
  Args,
  Identifier,
  Literal,
  Operator,

  // Temporary
  Chisato,

  // Error
  Bad(&'static str),
}

pub struct AstNode<'a> {
  token: Token<'a>,
  kind: AstKind,
  children: Vec<AstNode<'a>>,
}

impl<'a> std::ops::Index<usize> for AstNode<'a> {
  type Output = AstNode<'a>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.children[index]
  }
}
impl<'a> AstNode<'a> {
  pub fn new(token: Token<'a>, kind: AstKind) -> AstNode<'a> {
    AstNode {
      token,
      kind,
      children: Vec::new(),
    }
  }
  pub fn add_node(&mut self, node: AstNode<'a>) -> &mut AstNode<'a> {
    self.children.push(node);
    self.children.last_mut().unwrap()
  }
  pub fn get_token(&self) -> &Token<'a> {
    &self.token
  }
  pub fn get_kind(&self) -> AstKind {
    self.kind
  }
  pub fn set_kind(&mut self, kind: AstKind) {
    self.kind = kind;
  }
}

impl AstNode<'_> {
  pub fn new_root() -> AstNode<'static> {
    AstNode {
      token: Token::new_head(),
      kind: AstKind::Root,
      children: Vec::new(),
    }
  }
}

pub struct Ast<'a> {
  root: AstNode<'a>,
}

impl<'a> std::ops::Index<usize> for Ast<'a> {
  type Output = AstNode<'a>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.root[index]
  }
}
impl<'a> Ast<'a> {
  pub fn new() -> Ast<'a> {
    Ast {
      root: AstNode::new_root(),
    }
  }
  pub fn add_node(&mut self, node: Token<'a>) -> &mut AstNode<'a> {
    self.root.add_node(AstNode::new(node, AstKind::Fun))
  }
  pub fn get_mut_root(&mut self) -> &mut AstNode<'a> {
    &mut self.root
  }
}