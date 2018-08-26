extern crate env_logger;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use std::error::Error;

#[derive(Debug)]
struct Link {
    link: String,
    text: String,
}

impl Link {
    fn from_node<'a>(node: select::node::Node<'a>) -> Link {
        let text = node.text().replace(". [discuss]", "");
        let link = node.find(Name("a")).next().unwrap().attr("href").unwrap();
        Link {
            link: link.to_string(),
            text: text,
        }
    }
}

#[derive(Debug)]
struct CrateOfWeek {
    name: String,
    text: String,
    link: String,
}

#[derive(Debug)]
struct Article {
    news: Vec<Link>,
    crate_of_week: CrateOfWeek,
    updates: Vec<Link>,
}

fn parse_crate_of_week(document: &Document) -> Result<CrateOfWeek, Box<dyn Error>> {
    let p = document
        .find(Attr("id", "crate-of-the-week"))
        .next()
        .unwrap()
        .next()
        .unwrap()
        .next()
        .unwrap();

    let text = p.text();
    let link = p
        .find(Name("a"))
        .take(1)
        .map(|node| (node.attr("href").unwrap(), node.text()))
        .collect::<Vec<_>>();

    let (link, name) = link.get(0).unwrap();

    Ok(CrateOfWeek {
        name: name.to_string(),
        text: text.to_string(),
        link: link.to_string(),
    })
}

fn parse_news(doc: &Document) -> Result<Vec<Link>, Box<dyn Error>> {
    Ok(doc
        .find(Attr("id", "news-blog-posts"))
        .next()
        .unwrap()
        .next()
        .unwrap()
        .next()
        .unwrap()
        .find(Name("ul").descendant(Name("li")))
        .map(Link::from_node)
        .collect())
}

fn parse_updates_from_core(doc: &Document) -> Result<Vec<Link>, Box<dyn Error>> {
    Ok(
        doc
        .find(Attr("id", "updates-from-rust-core"))
        .next().unwrap()
        .next().unwrap()
        .next().unwrap()
        .next().unwrap()
        .next().unwrap()
        .find(Name("ul").descendant(Name("li")))
        .map(Link::from_node)
        .collect()
    )
}

fn parse_article(link: &str) -> Result<Article, Box<dyn Error>> {
    let html = reqwest::get(link)?.text()?;
    let document = Document::from(html.as_str());

    let news = parse_news(&document)?;
    let crate_of_week = parse_crate_of_week(&document)?;
    let updates = parse_updates_from_core(&document)?;

    Ok(Article {
        news,
        crate_of_week,
        updates,
    })
}

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
        println!("Fetching #{} — {}", id, link);
        let res = parse_article(link)?;

        println!("{:?}", res);
    }

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}
