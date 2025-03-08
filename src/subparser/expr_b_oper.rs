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

add_subparser!(expr_b_equal, BinOper);
add_subparser!(expr_b_nequal, BinOper);
add_subparser!(expr_b_less, BinOper);
add_subparser!(expr_b_lesseq, BinOper);
add_subparser!(expr_b_greater, BinOper);
add_subparser!(expr_b_greatereq, BinOper);
add_subparser!(expr_b_dot, BinOper);
add_subparser!(expr_b_ddot, BinOper);
add_subparser!(expr_b_and, BinOper);
add_subparser!(expr_b_or, BinOper);
add_subparser!(expr_b_xor, BinOper);
add_subparser!(expr_b_lshift, BinOper);
add_subparser!(expr_b_rshift, BinOper);
add_subparser!(expr_b_plus, BinOper);
add_subparser!(expr_b_minus, BinOper);
add_subparser!(expr_b_asterisk, BinOper);
add_subparser!(expr_b_slash, BinOper);
add_subparser!(expr_b_mod, BinOper);
add_subparser!(expr_b_pluseq, BinOper);
add_subparser!(expr_b_minuseq, BinOper);
add_subparser!(expr_b_asteriskeq, BinOper);
add_subparser!(expr_b_slasheq, BinOper);
add_subparser!(expr_b_modeq, BinOper);
add_subparser!(expr_b_andeq, BinOper);
add_subparser!(expr_b_oreq, BinOper);
add_subparser!(expr_b_xoreq, BinOper);
add_subparser!(expr_b_lshifteq, BinOper);
add_subparser!(expr_b_rshifteq, BinOper);
add_subparser!(expr_b_dand, BinOper);
add_subparser!(expr_b_dor, BinOper);
add_subparser!(expr_b_dequal, BinOper);
