extern crate env_logger;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use std::error::Error;

mod parsers;
mod types;

use parsers::*;

fn run() -> Result<(), Box<dyn Error>> {
    let last_id = 245;

    let html = reqwest::get("https://this-week-in-rust.org")?.text()?;

    let document = Document::from(html.as_str());

    let links = parse_home_page(&document, last_id);

    for (ref id, link) in links {
        println!("\n———\nFetching #{} — {}", id, link);
        let res = parse_article(link, *id)?;

        println!("{}", res);
    }

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}
