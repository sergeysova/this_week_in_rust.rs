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

    let links: Vec<(i32, &str)> = document
        .find(Class("post-title").descendant(Name("a")))
        .map(|node| {
            let title = node.text();
            let numb = title.split(" ").last().unwrap();

            (
                numb.to_string().parse().unwrap(),
                node.attr("href").unwrap(),
            )
        })
        .filter(|(id, _)| *id > last_id)
        .collect();

    for (id, link) in links {
        println!("\n———\nFetching #{} — {}", id, link);
        let res = parse_article(link, id)?;

        println!("{}", res);
    }

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}
