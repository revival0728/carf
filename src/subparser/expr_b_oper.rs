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
        if !super::super::add_a_exprlike(parser, node) { return None }
        Some(AstKind::PushToStk)
      }
    }
  };
}

add_subparser!(expr_b_equal, Operator);
add_subparser!(expr_b_nequal, Operator);
add_subparser!(expr_b_less, Operator);
add_subparser!(expr_b_lesseq, Operator);
add_subparser!(expr_b_greater, Operator);
add_subparser!(expr_b_greatereq, Operator);
add_subparser!(expr_b_dot, Operator);
add_subparser!(expr_b_ddot, Operator);
add_subparser!(expr_b_and, Operator);
add_subparser!(expr_b_or, Operator);
add_subparser!(expr_b_xor, Operator);
add_subparser!(expr_b_lshift, Operator);
add_subparser!(expr_b_rshift, Operator);
add_subparser!(expr_b_plus, Operator);
add_subparser!(expr_b_minus, Operator);
add_subparser!(expr_b_asterisk, Operator);
add_subparser!(expr_b_slash, Operator);
add_subparser!(expr_b_mod, Operator);
add_subparser!(expr_b_pluseq, Operator);
add_subparser!(expr_b_minuseq, Operator);
add_subparser!(expr_b_asteriskeq, Operator);
add_subparser!(expr_b_slasheq, Operator);
add_subparser!(expr_b_modeq, Operator);
add_subparser!(expr_b_andeq, Operator);
add_subparser!(expr_b_oreq, Operator);
add_subparser!(expr_b_xoreq, Operator);
add_subparser!(expr_b_lshifteq, Operator);
add_subparser!(expr_b_rshifteq, Operator);
add_subparser!(expr_b_dand, Operator);
add_subparser!(expr_b_dor, Operator);
add_subparser!(expr_b_dequal, Operator);