use chrono::prelude::*;

pub fn construct_calendar_url(query: &str) -> String {
  if query == "cal" {
    let google_calendar = "https://calendar.google.com/calendar/u/0/r/";

    google_calendar.to_string()
  } else {
    construct_calendar_url_date(&query[4..])
  }
}

pub fn construct_calendar_url_date(query: &str) -> String {
  let mut split_query = query.split_whitespace();
  let display = match split_query.next().unwrap() {
    "d" => "day",
    "w" => "week",
    "m" => "month",
    "y" => "year",
    _ => "month"
  };

  let current_time = Local::now();

  let twitter_search_url = format!("https://calendar.google.com/calendar/u/0/r/{}/{}/{}/{}", 
      display,
      split_query.next().unwrap_or(&current_time.year().to_string()[..]), 
      split_query.next().unwrap_or(&current_time.month().to_string()[..]),
      split_query.next().unwrap_or(&current_time.day().to_string()[..]));

  twitter_search_url
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_calendar_url_date() {
      let fake_query = "d 2020 11 3";
      let expected_value = "https://calendar.google.com/calendar/u/0/r/day/2020/11/3";

      assert_eq!(construct_calendar_url_date(fake_query), expected_value);
  }

  #[test]
  fn test_construct_calendar_url() {
    let fake_query = "cal d 2020 11 3";
    let expected_value = "https://calendar.google.com/calendar/u/0/r/day/2020/11/3";

    assert_eq!(construct_calendar_url(fake_query), expected_value);
  }
}