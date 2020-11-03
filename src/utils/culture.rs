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
  fn test_construct_culture_by_id() {
      let id = "177013";
      assert_eq!(construct_culture_by_id(id), "https://nhentai.net/g/177013/");
  }

  #[test]
  fn test_construct_culture_by_tags() {
      let id = "love live";
      assert_eq!(construct_culture_by_tags(id), "https://nhentai.net/search/?q=love+live");
  }

  #[test]
  fn test_construct_culture_with_tags() {
      let tags = "nh love live";
      assert_eq!(construct_culture_url(tags), "https://nhentai.net/search/?q=love+live");
  }

  #[test]
  fn test_construct_culture_with_id() {
      let id = "nh 177013";
      assert_eq!(construct_culture_url(id), "https://nhentai.net/g/177013/");
  }
}