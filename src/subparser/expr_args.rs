use crate::{ast::{token::TokenKind, AstKind, AstNode}, parser::Parser};

pub fn expr_args<'a>() -> super::SubParser<'a> {
  super::SubParser::new(parse)
}

fn parse<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> Option<AstKind> {
  loop {
    parser.parse_fn(&mut AstNode::new_empty());
    match parser.lexer_peek() {
      Some(token) => {
        if token.get_kind() != TokenKind::Comma {
          break;
        }
      },
      None => {
        super::error(node, "Expected Expr or Identifier");
        return None
      }
    }
  }
  Some(AstKind::GrapArgs)
}