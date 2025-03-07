use std::rc::Rc;
use crate::ast::token::{Token, TokenKind};
use crate::ast::{Ast, AstKind, AstNode};
use crate::lexer::Lexer;
use crate::subparser::{SubParser, SubParserList};
use std::collections::BTreeMap;

type ExpectList<'a> = BTreeMap<&'a str, Vec<TokenKind>>;
type KindIdMap<'a> = BTreeMap<TokenKind, &'a str>;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  expect_list: ExpectList<'a>,
  kind_id_map: KindIdMap<'a>,
  subparser: SubParserList<'a>,
  ast: Ast<'a>,
}

impl<'a> Parser<'a> {
  fn get_expect_list() -> ExpectList<'a> {
    let mut expect_list = BTreeMap::new();
    use TokenKind::*;
    macro_rules! kind_list {
      ($kind:expr) => {
        {
          let mut k = Vec::new();
          k.push($kind);
          k
        }
      };
      ($kind:expr, $($kinds:expr),*) => {
        {
          let mut k = Vec::new();
          k.push($kind);
          k.append(&mut kind_list![$($kinds),*]);
          k
        }
      };
    }
    macro_rules! add_expect {
      ($name:literal, $($kinds:expr),+) => {
        expect_list.insert($name, kind_list![$($kinds),+]);
      };
    }
    add_expect!("fun", Fun, Identifier, LParen, Union(0), RParen, Colon, Identifier, Union(1));
    add_expect!("stmt_multi", LStmt, Union(0), RStmt);
    add_expect!("stmt_let", Let, Identifier, Colon, Identifier, Equal, Union(0));
    add_expect!("stmt_while", While, LParen, Union(0), RParen, Union(1));
    add_expect!("stmt_if", If, LParen, Union(0), RParen, Union(1));
    add_expect!("stmt_else", Else, Union(0));
    add_expect!("stmt_continue", Continue);
    add_expect!("stmt_break", Break);
    add_expect!("stmt_return", Return);
    add_expect!("stmt_for", For, Identifier, In, Union(0), Union(1));
    add_expect!("expr_char", Char);
    add_expect!("expr_paren", LParen, Union(0), RParen); // TODO: Tell it is expr or function call
    add_expect!("expr_indexing", LIndex, Union(0), RIndex);
    add_expect!("expr_u_minus", Minus, Union(0));
    add_expect!("expr_identifer", Identifier);
    add_expect!("expr_integer", Int);
    add_expect!("expr_float", Float);
    add_expect!("expr_string", String);

    //TODO: Check if Ast is correct
    add_expect!("expr_u_not", Not);
    add_expect!("expr_u_dplus", DPlus);
    add_expect!("expr_u_dminus", DMinus);
    add_expect!("expr_b_equal", Equal);
    add_expect!("expr_b_plus", Plus);
    add_expect!("expr_b_minus", Minus);
    add_expect!("expr_b_asterisk", Asterisk);
    add_expect!("expr_b_slash", Slash);
    add_expect!("expr_b_mod", Mod);
    add_expect!("expr_b_pluseq", PlusEq);
    add_expect!("expr_b_minuseq", MinusEq);
    add_expect!("expr_b_asteriskeq", AsteriskEq);
    add_expect!("expr_b_slasheq", SlashEq);
    add_expect!("expr_b_modeq", ModEq);
    add_expect!("expr_b_and", And);
    add_expect!("expr_b_or", Or);
    add_expect!("expr_b_xor", Xor);
    add_expect!("expr_b_andeq", AndEq);
    add_expect!("expr_b_oreq", OrEq);
    add_expect!("expr_b_xoreq", XorEq);
    add_expect!("expr_b_lshift", LShift);
    add_expect!("expr_b_rshift", RShift);
    add_expect!("expr_b_lshifteq", LShiftEq);
    add_expect!("expr_b_rshifteq", RShiftEq);
    add_expect!("expr_b_dand", DAnd);
    add_expect!("expr_b_dor", DOr);
    add_expect!("expr_b_dequal", DEqual);
    add_expect!("expr_b_nequal", NEqual);
    add_expect!("expr_b_greater", Greater);
    add_expect!("expr_b_less", Less);
    add_expect!("expr_b_greatereq", GreaterEq);
    add_expect!("expr_b_lesseq", LessEq);
    add_expect!("expr_b_dot", Dot);
    add_expect!("expr_b_ddot", DDot);

    expect_list
  }
  fn get_kind_id_map(expect_list: Rc<&ExpectList<'a>>) -> KindIdMap<'a> {
    let mut kind_id_map = BTreeMap::new();
    for (id, kinds) in expect_list.iter() {
      if kinds[0] != TokenKind::Union(0) || kinds.len() == 1 {
        kind_id_map.insert(kinds[0], *id);
      } else {
        kind_id_map.insert(kinds[1], *id);
      }
    }

    kind_id_map
  }
  pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
    let expect_list = Self::get_expect_list();
    let kind_id_map = Self::get_kind_id_map(Rc::new(&expect_list));
    let subparser = SubParser::get_subparser_list();
    let ast = Ast::new();

    Parser {
      lexer,
      expect_list,
      kind_id_map,
      subparser,
      ast,
    }
  }
}

impl<'a> Parser<'a> {
  pub fn parse(&mut self) -> &Ast<'a> {
    let mut new_ast = Ast::new();
    let current = new_ast.get_mut_root();
    loop {
      let token_option = self.lexer.next();
      match token_option {
        Some(token) => {
          match token.get_kind() {
            TokenKind::Fun => {
              let new_node = current.add_node(AstNode::new(token, AstKind::Fun));
              let subparser = *self.subparser.get("fun").unwrap();
              subparser.parse(self, new_node);
            },
            TokenKind::Let => {
              let new_node = current.add_node(AstNode::new(token, AstKind::Let));
              let subparser = *self.subparser.get("stmt_let").unwrap();
              subparser.parse(self, new_node);
            },
            _ => panic!("Invalid token (Parser::parse)"),
          }
        },
        None => break,
      }
    }
    self.ast = new_ast;

    &self.ast
  }
  pub fn parse_fn(&mut self, node: &mut AstNode<'a>) {
    let current = node;
    loop {
      let token_option = self.lexer.next();
      // TODO: Find SemiColon and let it next token be new stmt
      match token_option {
        Some(token) => {
          let kind = token.get_kind();
          let id = match self.kind_id_map.get(&kind) {
            Some(id) => *id,
            None => panic!("Invalid token (Parser::parse_fn)"),
          };
          let subparser = *self.subparser.get(id).unwrap();
          let new_node = current.add_node(AstNode::new(token, AstKind::Chisato));
          subparser.parse(self, new_node);
        },
        None => break,
      }
    }
  }
}

impl<'a> Parser<'a> {
  pub fn get_expect(&self, id: &str) -> Vec<TokenKind> {
    self.expect_list.get(id).unwrap().clone()
  }
  pub fn subparse(&mut self, id: &TokenKind, node: &mut AstNode<'a>) {
    let id = *self.kind_id_map.get(id).unwrap();
    let subparser = *self.subparser.get(id).unwrap();
    subparser.parse(self, node);
  }
  pub fn lexer_next(&mut self) -> Option<Token<'a>> {
    self.lexer.next()
  }
  pub fn lexer_peek(&mut self) -> Option<&Token<'a>> {
    self.lexer.peek()
  }
}

#[cfg(test)]
impl<'a> Parser<'a> {
  pub fn expose_kind_id_map() -> KindIdMap<'a> {
    let expect_list = Self::get_expect_list();
    Self::get_kind_id_map(Rc::new(&expect_list))
  }
  pub fn expose_expect_list() -> ExpectList<'a> {
    Self::get_expect_list()
  }
}