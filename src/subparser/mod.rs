mod expr_paren;
mod expr_b_oper;
use expr_b_oper::*;
mod expr_ua_oper;
use expr_ua_oper::*;
mod expr_ub_oper;
use expr_ub_oper::*;
mod expr_var;
use expr_var::*;
mod expr_args;
mod expr_indexing;

use std::collections::BTreeMap;
use crate::parser::Parser;
use crate::ast::{AstNode, AstKind};

pub type SubParserList<'a> = BTreeMap<&'static str, SubParser<'a>>;

type ParseToken<'a> = fn(&mut Parser<'a>, &mut AstNode<'a>) -> Option<AstKind>;

pub fn error(node: &mut AstNode, msg: &'static str) {
  node.set_kind(AstKind::Bad(msg));
}

// Add an Expr to node or report error
pub fn add_a_exprlike<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> bool {
  let new_node = node.add_node(AstNode::new_empty());
  parser.parse_fn(new_node);
  if new_node.child_count() > 0 {
    error(node, "Expected Expr or Identifier");
    return false;
  }
  node.add_node(match parser.pop_node() {
    Some(node) => node,
    None => {
      error(node, "Expected Expr or Identifier");
      return false;
    }
  });

  true
}

pub fn add_b_exprlike<'a>(parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> bool {
  match parser.pop_node() {
    Some(expr_like) => {
      node.add_node(expr_like);
    },
    None => {
      error(node, "Expected an Identifier or Expr");
      return false;
    }
  };

  true
}

#[derive(Copy, Clone)]
pub struct SubParser<'a> {
  parse_token: ParseToken<'a>,
}

impl<'a> SubParser<'a> {
  pub fn parse(&self, parser: &mut Parser<'a>, node: &mut AstNode<'a>) -> Option<AstKind> { 
    (self.parse_token)(parser, node) 
  }
}

impl<'a> SubParser<'a> {
  pub fn new(
    parse_token: ParseToken,
  ) -> SubParser {

    SubParser {
      parse_token,
    }
  }

  pub fn get_subparser_list() -> SubParserList<'a> {
    let mut subparser_list = BTreeMap::new();
    macro_rules! add_subparser {
      ($parser:ident) => {
        subparser_list.insert(stringify!($parser), $parser::$parser());
      };
    }
    add_subparser!(expr_paren);
    add_subparser!(expr_identifer);
    add_subparser!(expr_int);
    add_subparser!(expr_float);
    add_subparser!(expr_char);
    add_subparser!(expr_string);
    add_subparser!(expr_b_equal);
    add_subparser!(expr_b_nequal);
    add_subparser!(expr_b_less);
    add_subparser!(expr_b_lesseq);
    add_subparser!(expr_b_greater);
    add_subparser!(expr_b_greatereq);
    add_subparser!(expr_b_dot);
    add_subparser!(expr_b_ddot);
    add_subparser!(expr_b_and);
    add_subparser!(expr_b_or);
    add_subparser!(expr_b_xor);
    add_subparser!(expr_b_lshift);
    add_subparser!(expr_b_rshift);
    add_subparser!(expr_b_plus);
    add_subparser!(expr_b_minus);
    add_subparser!(expr_b_asterisk);
    add_subparser!(expr_b_slash);
    add_subparser!(expr_b_mod);
    add_subparser!(expr_b_pluseq);
    add_subparser!(expr_b_minuseq);
    add_subparser!(expr_b_asteriskeq);
    add_subparser!(expr_b_slasheq);
    add_subparser!(expr_b_modeq);
    add_subparser!(expr_b_andeq);
    add_subparser!(expr_b_oreq);
    add_subparser!(expr_b_xoreq);
    add_subparser!(expr_b_lshifteq);
    add_subparser!(expr_b_rshifteq);
    add_subparser!(expr_b_dand);
    add_subparser!(expr_b_dor);
    add_subparser!(expr_b_dequal);
    add_subparser!(expr_u_dminus);
    add_subparser!(expr_u_dplus);
    add_subparser!(expr_u_minus);
    add_subparser!(expr_u_not);
    add_subparser!(expr_indexing);

    subparser_list
  }
}