extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
.add(b'>').add(b'`');

pub fn construct_jisho_url(query: &str) -> String {
  if query == "jp" {
    let jisho_dotorg = "https://jisho.org";

    jisho_dotorg.to_string()
  } else {
    let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
    construct_jisho_url_by_query(&encoded_query)
  }
}

pub fn construct_jisho_url_by_query(query: &str) -> String {
  format!("https://jisho.org/search/{}", query)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_jisho_url_by_query() {
      let query = utf8_percent_encode("stir fry", FRAGMENT).to_string();
      assert_eq!(construct_jisho_url_by_query(&query), "https://jisho.org/search/stir%20fry");
  }

  #[test]
  fn test_construct_jisho_url() {
      let query = "jp stir fry";
      assert_eq!(construct_jisho_url(query), "https://jisho.org/search/stir%20fry");
  }
}