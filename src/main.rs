extern crate env_logger;
extern crate kuchiki;
extern crate reqwest;
extern crate select;

// use kuchiki::traits::TendrilSink;
// use kuchiki::iter::*;

// fn run() -> Result<(), Box<std::error::Error>> {
//     let mut res = reqwest::get("https://this-week-in-rust.org")?;

//     let last_id = 247;

//     println!("{} — {}", res.url(), res.status());
//     let html = res.text()?;
//     let document = kuchiki::parse_html().one(html);

//     let _: Vec<_> = document
//         .select(".row.post-title")
//         .unwrap()
//         .map(|row| {
//             let node = row.as_node();
//             node.select("a")
//                 .unwrap()
//                 .map(|a| {
//                     let node = a.as_node();
//                     println!("{}", node.to_string());
//                     node
//                 })
//                 .next()
//                 .unwrap()
//         })
//         .filter(|node| {
//             node
//         })
//         .collect();

//     Ok(())
// }

use reqwest::StatusCode;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn run() -> Result<(), Box<std::error::Error>> {
    let mut res = reqwest::get("https://this-week-in-rust.org")?;

    let last_id = 247;

    match res.status() {
        StatusCode::Ok => {}
        _ => panic!("Got not OK"),
    }
    let html = res.text()?;

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
        println!("{} — {}", id, link);
    }

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}
