#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "cal" => utils::calendar::construct_calendar_url(&cmd),
        "nh" => utils::culture::construct_culture_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}