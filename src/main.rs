extern crate env_logger;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

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
struct Article {
    news: Vec<Link>,
}

fn parse_article(link: &str) -> Result<Article, Box<std::error::Error>> {
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

    Ok(Article { news })
}

fn run() -> Result<(), Box<std::error::Error>> {
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

        println!("{}", res);
    }

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}
