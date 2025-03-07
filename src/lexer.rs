use core::panic;
use std::collections::VecDeque;

use crate::ast::token::{self, TokenKind};
use crate::{ast::token::Token, utilities::trie::Trie};

pub struct Lexer<'a> {
  tokens: VecDeque<Token<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Lexer<'a> {
    let mut token_trie = Trie::new();
    for t in token::SYMBOL_LIST.iter() {
      token_trie.insert(t);
    }
    for t in token::KEYWORD_LIST.iter() {
      token_trie.insert(t);
    }

    let mut tokens = VecDeque::new();
    {
      let mut last_pos = 0_usize;
      let mut state = 0_u8;
      let s = source;
      let deter_state = |i: usize, c: char| -> u8 {
        if token_trie.contains(&s[i..=i]) { 
          1 
        } else if c == '"' { 
          2 
        } else if c == '\'' {
          3
        } else if c.is_digit(10) {
          4
        } else {
          5
        }
      };
      for (i, c) in s.chars().enumerate() {
        if state == 0 && (c == ' ' || c == '\n' || c == '\t' || c == '\r') {
          continue;
        }
        match state {
          0 => {
            // Empty state
            state = deter_state(i, c);
            last_pos = i;
          },
          1 => {
            // Symbol or Keyword
            if !token_trie.contains(&s[last_pos..=i]) {
              tokens.push_back(Token::from_token(&s[last_pos..=i]));
              state = deter_state(i, c);
              last_pos = i;
            }
          },
          2 => {
            // String Literal
            if c == '"' {
              match token::literal_token_map(&s[last_pos..=i]) {
                TokenKind::String => {
                  tokens.push_back(Token::new(TokenKind::String, &s[last_pos..=i]));
                  state = 0;
                },
                _ => panic!("Compiler Error at lexing string literal"),
              }
            }
          },
          3 => {
            // Char Literal
            if c == '\'' {
              match token::literal_token_map(&s[last_pos..=i]) {
                TokenKind::Char => {
                  tokens.push_back(Token::new(TokenKind::Char, &s[last_pos..=i]));
                  state = 0;
                },
                _ => panic!("Compiler Error at lexing char literal"),
              }
            }
          },
          4 => {
            // Number Literal
            match token::literal_token_map(&s[last_pos..=i]) {
              TokenKind::Other => {
                match token::literal_token_map(&s[last_pos..i]) {
                  TokenKind::Int => {
                    tokens.push_back(Token::new(TokenKind::Int, &s[last_pos..i]));
                    state = deter_state(i, c);
                    last_pos = i;
                  },
                  TokenKind::Float => {
                    tokens.push_back(Token::new(TokenKind::Float, &s[last_pos..i]));
                    state = deter_state(i, c);
                    last_pos = i;
                  },
                  _ => panic!("Compiler Error at lexing number literal (IM)"),
                }
              }
              _ => panic!("Compiler Error at lexing number literal (OM)"),
            }
          }
          5 => {
            // Identifier
            if token_trie.contains(&s[last_pos..=i]) {
              tokens.push_back(Token::new(token::TokenKind::Identifier, &s[last_pos..i]));
              state = deter_state(i, c);
              last_pos = i;
            }
          }
          _ => panic!("Invalid state"),
        }
        
      }
      // TODO: more detailed CE message
      if state != 0 {
        tokens.push_back(Token::new_bad(
          match state {
            1 => "Symbol or Keyword",
            2 => "String Literal",
            3 => "Char Literal",
            4 => "Number Literal",
            5 => "Identifier",
            _ => "Unknown",
          }
        ));
      }
      tokens.push_back(Token::new_eof());
    }

    Lexer {
      tokens,
    }
  }
  pub fn next(&mut self) -> Option<Token<'a>> {
    self.tokens.pop_back()
  }
  pub fn peek(&self) -> Option<&Token<'a>> {
    self.tokens.front()
  }
}