use core::panic;

use crate::{ast::{token::TokenKind, AstKind, AstNode}, parser::Parser};

pub fn expr_paren<'a>() -> super::SubParser<'a> {
  super::SubParser::new(parse)
}

fn parse<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) {
  node.set_kind(AstKind::Paren);
  let token = node.get_token();
  let expect = parser.get_expect("expr");
  assert_eq!(token.get_kind(), expect[0]);
  for e_kind in expect {
    let n_token = match parser.lexer_next() {
      Some(token) => token,
      None => panic!("Invalid token (expr:1)"),
    };
    if n_token.get_kind() != e_kind {
      error(parser, node);
      return;
    }
    match e_kind {
      TokenKind::Union(_) => { 
        let new_node = node.add_node(AstNode::new(n_token, AstKind::Chisato));
        parser.subparse(&e_kind, new_node); 
      },
      TokenKind::RParen => { 
        node.add_node(AstNode::new(n_token, AstKind::Expr)); 
      },
      _ => panic!("Invalid token (expr:2)"),
    }
  }
}
fn error(_parser: &mut Parser, node: &mut AstNode) {
  node.set_kind(AstKind::Bad("Error while parsing expr"));
}