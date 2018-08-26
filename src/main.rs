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

impl<'a> Into<Link> for select::node::Node<'a> {
    fn into(self) -> Link {
        let text = self.text().replace(". [discuss]", "");
        let link = self.find(Name("a")).next().unwrap().attr("href").unwrap();
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

fn parse_article(link: &str) -> Result<Article, Box<dyn Error>> {
    let html = reqwest::get(link)?.text()?;
    let document = Document::from(html.as_str());

    let news: Vec<Link> = document
        .find(Attr("id", "news-blog-posts"))
        .next()
        .unwrap()
        .next()
        .unwrap()
        .next()
        .unwrap()
        .find(Name("ul").descendant(Name("li")))
        .map(|node| node.into())
        .collect();

    let crate_of_week = parse_crate_of_week(&document)?;

    Ok(Article { news, crate_of_week })
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
