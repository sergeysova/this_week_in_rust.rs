extern crate env_logger;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use std::error::Error;

use types::*;

pub fn parse_crate_of_week(document: &Document) -> Result<CrateOfWeek, Box<dyn Error>> {
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

pub fn parse_news(doc: &Document) -> Result<Vec<Link>, Box<dyn Error>> {
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

pub fn parse_updates_from_core(doc: &Document) -> Result<Vec<Link>, Box<dyn Error>> {
    Ok(doc
        .find(Attr("id", "updates-from-rust-core"))
        .next()
        .unwrap()
        .next()
        .unwrap()
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

pub fn parse_article_date(doc: &Document) -> Result<String, Box<dyn Error>> {
    Ok(doc
        .find(Class("time-prefix"))
        .next()
        .unwrap()
        .text()
        .trim()
        .to_string())
}

pub fn parse_article(link: &str, id: i32) -> Result<Article, Box<dyn Error>> {
    let html = reqwest::get(link)?.text()?;
    let document = Document::from(html.as_str());

    let date = parse_article_date(&document)?;
    let news = News::new(parse_news(&document)?);
    let crate_of_week = parse_crate_of_week(&document)?;
    let updates = Updates::new(parse_updates_from_core(&document)?);

    Ok(Article {
        id,
        link: link.to_string(),
        date,
        news,
        crate_of_week,
        updates,
    })
}

pub fn parse_home_page(doc: &Document, last_id: i32) -> Vec<(i32, &str)> {
    doc.find(Class("post-title").descendant(Name("a")))
        .map(|node| {
            let title = node.text();
            let numb = title.split(" ").last().unwrap();

            (
                numb.to_string().parse().unwrap(),
                node.attr("href").unwrap(),
            )
        })
        .filter(|(id, _)| *id > last_id)
        .collect()
}
