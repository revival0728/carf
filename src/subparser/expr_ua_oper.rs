macro_rules! add_subparser {
  ($name:ident, $kind:ident) => {
    pub mod $name {
      use crate::{ast::{AstKind, AstNode}, parser::Parser, subparser::SubParser};
      pub fn $name<'a>() -> SubParser<'a> {
        SubParser::new(parse)
      }

      fn parse<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> Option<AstKind> {
        node.set_kind(AstKind::$kind);
        if !super::super::add_b_exprlike(parser, node) { return None }
        Some(AstKind::PushToStk)
      }
    }
  }
}

add_subparser!(expr_u_dminus, Operator);
add_subparser!(expr_u_dplus, Operator);