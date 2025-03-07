use core::panic;

use crate::{ast::{token::{Token, TokenKind}, AstKind, AstNode}, parser::Parser};

pub fn expr_paren<'a>() -> super::SubParser<'a> {
  super::SubParser::new(parse)
}

fn parse<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) {
  let token = node.get_token();
  let expect = parser.get_expect("expr_paren");
  assert_eq!(token.get_kind(), expect[0]);
  for e_kind in expect {
    match e_kind {
      TokenKind::Union(0) => {
        let new_node = node.add_node(AstNode::new(Token::new_empty(), AstKind::Chisato));
        parser.parse_fn(new_node);
      },
      TokenKind::RParen => {
        let n_token = match parser.lexer_next() {
          Some(token) => token,
          None => panic!("Invalid token (expr:1)"),
        };
        let n_kind = n_token.get_kind();
        if n_kind != TokenKind::RParen {
          node.set_kind(AstKind::Bad("Expected ')'"));
        }
      },
      _ => panic!("Grammer for expr_paren ERROR."),
    }
  }
  if node[0].get_kind() == AstKind::Args {
    node.set_kind(AstKind::Call);
  } else if node[0].get_kind() == AstKind::Expr {
    node.set_kind(AstKind::Expr);
  } else {
    node.set_kind(AstKind::Bad("Expected Args or Expr"));
  }
}
fn error(_parser: &mut Parser, node: &mut AstNode) {
  node.set_kind(AstKind::Bad("Error while parsing expr"));
}