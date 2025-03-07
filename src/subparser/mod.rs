mod expr_paren;
mod basic;
use basic::*;

use std::collections::BTreeMap;
use crate::parser::Parser;
use crate::ast::AstNode;

pub type SubParserList<'a> = BTreeMap<&'static str, SubParser<'a>>;

type ParseToken<'a> = fn(&mut Parser<'a>, &mut AstNode<'a>);

#[derive(Copy, Clone)]
pub struct SubParser<'a> {
  parse_token: ParseToken<'a>,
}

impl<'a> SubParser<'a> {
  pub fn parse(&self, parser: &mut Parser<'a>, node: &mut AstNode<'a>) { 
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
    add_subparser!(expr_u_not);
    add_subparser!(expr_u_dminus);
    add_subparser!(expr_u_dplus);
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

    subparser_list
  }
}