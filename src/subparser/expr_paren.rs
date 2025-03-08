use core::panic;

use crate::{ast::{token::TokenKind, AstKind, AstNode}, parser::Parser};

pub fn expr_paren<'a>() -> super::SubParser<'a> {
  super::SubParser::new(parse)
}

fn parse<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> Option<AstKind> {
  let token = node.get_token();
  let expect = parser.get_expect("expr_paren");
  assert_eq!(token.get_kind(), expect[0]);
  for e_kind in expect {
    match e_kind {
      TokenKind::LParen => continue,
      TokenKind::Union(0) => {
        if !super::add_a_exprlike(parser, node) {
          return None;
        }
      },
      TokenKind::RParen => {
        let n_token = match parser.lexer_next() {
          Some(token) => token,
          None => panic!("Invalid token (expr_paren:1)"),
        };
        let n_kind = n_token.get_kind();
        if n_kind != TokenKind::RParen {
          super::error(node, "Expected ')'");
        }
      },
      _ => panic!("Grammer for expr_paren Error"),
    }
  }
  assert_eq!(node.child_count(), 1);
  match node[0].get_kind() {
    AstKind::Args => node.set_kind(AstKind::Call),
    AstKind::Operator | AstKind::Expr => node.set_kind(AstKind::Expr),
    AstKind::Identifier | AstKind::Literal => node.set_kind(AstKind::ExprOrCall),
    _ => super::error(node, "Expected Args or Expr"),
  };

  Some(AstKind::PushToStk)
}