use crate::{ast::{token::TokenKind, AstKind, AstNode}, parser::Parser};

pub fn expr_indexing<'a>() -> super::SubParser<'a> {
  super::SubParser::new(parse)
}

fn parse<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> Option<AstKind> {
  let expect = parser.get_expect("expr_indexing");
  for e_kind in expect {
    match e_kind {
      TokenKind::LIndex => continue,
      TokenKind::Union(0) => {
        if !super::add_a_exprlike(parser, node) {
          return None;
        }
      },
      TokenKind::RIndex => {
        let n_token = match parser.lexer_next() {
          Some(token) => token,
          None => panic!("Invalid token (expr_indexing:1)"),
        };
        let n_kind = n_token.get_kind();
        if n_kind != TokenKind::RIndex {
          super::error(node, "Expected ']'");
        }
      },
      _ => panic!("Grammer for expr_indexing Error"),
    }
  }

  Some(AstKind::PushToStk)
}