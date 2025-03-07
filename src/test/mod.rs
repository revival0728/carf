#[cfg(test)]
mod tests {
  #[test]
  fn subparser_check() {
    use crate::subparser::SubParser;
    use crate::parser::Parser;

    let expect_list = Parser::expose_expect_list();
    let subparser_list = SubParser::get_subparser_list();
    
    let expect_keys: Vec<&str> = expect_list.into_keys().collect();
    let actual_keys: Vec<&str> = subparser_list.into_keys().collect();

    assert_eq!(expect_keys, actual_keys);
  }

  #[test]
  fn check_expect_list_key() {
    use crate::parser::Parser;

    let expect_list = Parser::expose_expect_list();
    let keys: Vec<&str> = expect_list.into_keys().collect();

    for k in keys {
      assert!(!k.contains("-"), "{}", k);
    }
  }

  #[test]
  fn check_kind_it_map() {
    use crate::parser::Parser;
    use crate::ast::token::TokenKind;

    let kind_id_map = Parser::expose_kind_id_map();
    let keys: Vec<TokenKind> = kind_id_map.clone().into_keys().collect();

    let mut cnt = 0;
    for k in keys {
      match k {
        TokenKind::Union(i) => {
          cnt += 1;
          assert!(i == 0 && cnt <= 1, "{:?}: {}", k, kind_id_map.get(&k).unwrap());
        },
        _ => {},
      }
    }
  }
}