#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum TokenKind {
  Identifier,

  // Symbols
  Equal,
  Plus,
  DPlus,
  Minus,
  DMinus,
  Asterisk,
  Slash,
  Mod,
  PlusEq,
  MinusEq,
  AsteriskEq,
  SlashEq,
  ModEq,
  And,
  Or,
  Not,
  Xor,
  LShift,
  RShift,
  LShiftEq,
  RShiftEq,
  DAnd,
  DOr,
  AndEq,
  OrEq,
  XorEq,
  DEqual,
  NEqual,
  Greater,
  Less,
  GreaterEq,
  LessEq,
  LParen,
  RParen,
  LIndex,
  RIndex,
  LStmt,
  RStmt,
  SemiColon,
  Colon,
  Comma,
  Dot,
  DDot,

  // Literal
  Int,
  Float,
  Char,
  String,

  // Keywords
  If,
  Else,
  While,
  Return,
  Break,
  Continue,
  For,
  In,
  Fun,
  Let,

  // Singals
  EOF,
  Bad(&'static str),
  Other,
  Head,
  Union(u8),
}

pub fn sym_token_map<'a>(token: &'a str) -> TokenKind {
  match token {
    "=" => TokenKind::Equal,
    "+" => TokenKind::Plus,
    "++" => TokenKind::DPlus,
    "-" => TokenKind::Minus,
    "--" => TokenKind::DMinus,
    "*" => TokenKind::Asterisk,
    "/" => TokenKind::Slash,
    "%" => TokenKind::Mod,
    "+=" => TokenKind::PlusEq,
    "-=" => TokenKind::MinusEq,
    "*=" => TokenKind::AsteriskEq,
    "/=" => TokenKind::SlashEq,
    "%=" => TokenKind::ModEq,
    "==" => TokenKind::DEqual,
    "!=" => TokenKind::NEqual,
    ">" => TokenKind::Greater,
    "<" => TokenKind::Less,
    ">=" => TokenKind::GreaterEq,
    "<=" => TokenKind::LessEq,
    "&" => TokenKind::And,
    "|" => TokenKind::Or,
    "!" => TokenKind::Not,
    "^" => TokenKind::Xor,
    "<<" => TokenKind::LShift,
    ">>" => TokenKind::RShift,
    "<<=" => TokenKind::LShiftEq,
    ">>=" => TokenKind::RShiftEq,
    "&&" => TokenKind::DAnd,
    "||" => TokenKind::DOr,
    "&=" => TokenKind::AndEq,
    "|=" => TokenKind::OrEq,
    "^=" => TokenKind::XorEq,
    "(" => TokenKind::LParen,
    ")" => TokenKind::RParen,
    "[" => TokenKind::LIndex,
    "]" => TokenKind::RIndex,
    "{" => TokenKind::LStmt,
    "}" => TokenKind::RStmt,
    ";" => TokenKind::SemiColon,
    ":" => TokenKind::Colon,
    "," => TokenKind::Comma,
    "." => TokenKind::Dot,
    ".." => TokenKind::DDot,
    _ => TokenKind::Other,
  }
}

pub fn keyword_token_map<'a>(token: &'a str) -> TokenKind {
  match token {
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "while" => TokenKind::While,
    "return" => TokenKind::Return,
    "break" => TokenKind::Break,
    "continue" => TokenKind::Continue,
    "for" => TokenKind::For,
    "in" => TokenKind::In,
    "fun" => TokenKind::Fun,
    "let" => TokenKind::Let,
    _ => TokenKind::Other,
  }
}

pub fn literal_token_map<'a>(token: &'a str) -> TokenKind {
  {
    // Check if token is an integer
    let mut is_int = true;
    for c in token.chars() {
      if !c.is_digit(10) {
        is_int = false;
        break;
      }
    }
    if is_int {
      return TokenKind::Int;
    }
  }
  {
    // Check if token is a float
    let mut is_float = true;
    let mut dot_count = 0_u32;
    for (i, c) in token.chars().enumerate() {
      if c == '.' && i == 0 {
        is_float = false;
        break;
      } else if c == '.' {
        dot_count += 1;
        if dot_count > 1 {
          is_float = false;
          break;
        }
      } else if !(c.is_digit(10) || (c == 'f' && i == token.len() - 1)) {
        is_float = false;
        break;
      }
    }
    if is_float {
      return TokenKind::Float;
    }
  }
  {
    // Check if token is a char
    let mut is_char = true;
    if token.len() == 3 {
      if token.chars().nth(0).unwrap() == '\'' && token.chars().nth(2).unwrap() == '\'' {
        is_char = true;
      }
    }
    if token.len() == 4 {
      if token.chars().nth(0).unwrap() == '\'' && token.chars().nth(3).unwrap() == '\'' {
        if token.chars().nth(1).unwrap() == '\\' {
          is_char = true;
        }
      }
    }
    if is_char {
      return TokenKind::Char;
    }
  }
  {
    // Check if token is a string
    let mut is_string = true;
    if token.len() >= 2 {
      if token.chars().nth(0).unwrap() == '"' && token.chars().nth(token.len() - 1).unwrap() == '"' {
        is_string = true;
      }
    }
    if is_string {
      return TokenKind::String;
    }
  }
  return TokenKind::Other;
}

pub const SYMBOL_LIST: [&str; 43] = [
  "=",
  "+",
  "++",
  "-",
  "--",
  "*",
  "/",
  "%",
  "+=",
  "-=",
  "*=",
  "/=",
  "%=",
  "==",
  "!=",
  ">",
  "<",
  ">=",
  "<=",
  "&&",
  "||",
  "&",
  "|",
  "!",
  "^",
  "<<",
  ">>",
  "<<=",
  ">>=",
  "&=",
  "|=",
  "^=",
  "(",
  ")",
  "[",
  "]",
  "{",
  "}",
  ",",
  ";",
  ":",
  ".",
  "..",
];

pub const KEYWORD_LIST: [&str; 10] = [
  "if",
  "else",
  "while",
  "return",
  "break",
  "continue",
  "for",
  "in",
  "fun",
  "let",
];

pub struct Token<'a> {
  kind: TokenKind,
  value: &'a str,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind, value: &'a str) -> Token<'a> {
    Token {
      kind,
      value,
    }
  }
  pub fn from_token(token: &'a str) -> Token<'a> {
    let mut kind = TokenKind::Other;
    kind = match kind {
      TokenKind::Other => sym_token_map(token),
      _ => kind,
    };
    kind = match kind {
      TokenKind::Other => keyword_token_map(token),
      _ => kind,
    };
    kind = match kind {
      TokenKind::Other => literal_token_map(token),
      _ => kind,
    };
    Token {
      kind,
      value: token,
    }
  }
  pub fn set_as_identifier(&mut self) {
    self.kind = TokenKind::Identifier;
  }
  pub fn set_as_eof(&mut self) {
    self.kind = TokenKind::EOF;
  }
  pub fn set_as_bad(&mut self, msg: &'static str) {
    self.kind = TokenKind::Bad(msg);
  }
}

impl Token<'_> {
  pub fn new_head() -> Token<'static> {
    Token {
      kind: TokenKind::Head,
      value: "",
    }
  }
  pub fn new_eof() -> Token<'static> {
    Token {
      kind: TokenKind::EOF,
      value: "",
    }
  }
  pub fn new_bad(msg: &'static str) -> Token<'static> {
    Token {
      kind: TokenKind::Bad(msg),
      value: "",
    }
  }
  pub fn get_kind(&self) -> TokenKind {
    self.kind
  }
}

impl Default for Token<'_> {
  fn default() -> Self {
    Token::new_head()
  }
}