macro_rules! add_subparser {
  ($name:ident, $kind:ident) => {
    pub mod $name {
      use crate::{ast::{AstKind, AstNode}, parser::Parser, subparser::SubParser};
      pub fn $name<'a>() -> SubParser<'a> {
        SubParser::new(parse)
      }

      fn parse<'a>(_parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> Option<AstKind> {
        node.set_kind(AstKind::$kind);
        Some(AstKind::PushToStk)
      }
    }
  };
}

add_subparser!(expr_identifer, Identifier);
add_subparser!(expr_int, Literal);
add_subparser!(expr_float, Literal);
add_subparser!(expr_char, Literal);
add_subparser!(expr_string, Literal);