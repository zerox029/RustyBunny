pub fn construct_culture_url(query: &str) -> String {
  if query == "nh" {
    let culture_dotcom = "https://nhentai.net";

    culture_dotcom.to_string()
  } else if query[3..].chars().all(char::is_numeric) {
    construct_culture_by_id(&query[3..])
  } else {
    construct_culture_by_tags(&query[3..])
  }
}

pub fn construct_culture_by_id(id: &str) -> String {
  format!("https://nhentai.net/g/{}/", id)
}

pub fn construct_culture_by_tags(tags: &str) -> String {
  let encoded_query = tags.replace(" ", "+");
  let culture_search_url = format!("https://nhentai.net/search/?q={}", encoded_query);

  culture_search_url
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_twitter_url() {
      let fake_query = "tw";
      assert_eq!(construct_twitter_url(fake_query), 
      "https://twitter.com");
  }

  #[test]
  fn test_construct_twitter_url_query() {
      let fake_query = "tw hello world";
      assert_eq!(construct_twitter_url(fake_query),     
      "https://twitter.com/search?q=hello%20world");
  }

  #[test]
  fn test_construct_twitter_url_profile() {
      let fake_query = "tw @fbOpenSource";
      assert_eq!(construct_twitter_url(fake_query), 
      "https://twitter.com/fbOpenSource");
  }

  #[test]
  fn test_construct_twitter_profile_url() {
      let fake_profile = "jsjoeio";
      assert_eq!(
          construct_twitter_profile_url(fake_profile),
          "https://twitter.com/jsjoeio"
      );
  }

  #[test]
  fn test_construct_twitter_search_url() {
      let fake_query = "hello world";
      assert_eq!(
          construct_twitter_search_url(fake_query),
          "https://twitter.com/search?q=hello%20world"
      );
  }
}