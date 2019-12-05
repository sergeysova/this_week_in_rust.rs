use failure::{Error, Fail};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use crate::html::escape;
use crate::types::*;

#[derive(Debug, Fail)]
pub enum ParseCrateOfWeekError {
    #[fail(display = "p not found")]
    ParagraphNotFound,

    #[fail(display = "link is not found")]
    LinkNotFound,
}

pub fn parse_crate_of_week(document: &Document) -> Result<CrateOfWeek, ParseCrateOfWeekError> {
    let p = document
        .find(Attr("id", "crate-of-the-week"))
        .next()
        .ok_or(ParseCrateOfWeekError::ParagraphNotFound)?
        .next()
        .ok_or(ParseCrateOfWeekError::ParagraphNotFound)?
        .next()
        .ok_or(ParseCrateOfWeekError::ParagraphNotFound)?;

    let text = p.text();
    let link = p
        .find(Name("a"))
        .take(1)
        .map(|node| {
            (
                node.attr("href").expect("node has no href attribute"),
                node.text(),
            )
        })
        .collect::<Vec<_>>();

    println!("{:?}", p);

    let (link, name) = link.get(0).ok_or(ParseCrateOfWeekError::LinkNotFound)?;

    Ok(CrateOfWeek {
        name: escape(name.to_string()),
        text: escape(text.to_string()),
        link: escape(link.to_string()),
    })
}

#[derive(Debug, Fail)]
pub enum ParseNewsError {
    #[fail(display = "#news-blog-posts not found")]
    IdNotFound,

    #[fail(display = "next element not found")]
    NextNotFound,
}

pub fn parse_news(doc: &Document) -> Result<Vec<Link>, ParseNewsError> {
    Ok(doc
        .find(Attr("id", "news-blog-posts"))
        .next()
        .ok_or(ParseNewsError::IdNotFound)?
        .next()
        .ok_or(ParseNewsError::NextNotFound)?
        .next()
        .ok_or(ParseNewsError::NextNotFound)?
        .find(Name("ul").descendant(Name("li")))
        .map(Link::from_node)
        .filter(Result::is_ok)
        .map(|v| v.expect("Here should be only Ok"))
        .collect())
}

#[derive(Debug, Fail)]
pub enum ParseUpdatesError {
    #[fail(display = "#updates-from-rust-core not found")]
    IdNotFound,

    #[fail(display = "next element not found")]
    NextNotFound,
}

pub fn parse_updates_from_core(doc: &Document) -> Result<Vec<Link>, ParseUpdatesError> {
    Ok(doc
        .find(Attr("id", "updates-from-rust-core"))
        .next()
        .ok_or(ParseUpdatesError::IdNotFound)?
        .next()
        .ok_or(ParseUpdatesError::NextNotFound)?
        .next()
        .ok_or(ParseUpdatesError::NextNotFound)?
        .next()
        .ok_or(ParseUpdatesError::NextNotFound)?
        .next()
        .ok_or(ParseUpdatesError::NextNotFound)?
        .find(Name("ul").descendant(Name("li")))
        .map(Link::from_node)
        .filter(Result::is_ok)
        .map(|v| v.expect("Here should be only ok"))
        .collect())
}

#[derive(Debug, Fail)]
pub enum ParseDateError {
    #[fail(display = ".time-prefix not found")]
    TimeNotFound,
}

pub fn parse_article_date(doc: &Document) -> Result<String, ParseDateError> {
    Ok(doc
        .find(Class("time-prefix"))
        .next()
        .ok_or(ParseDateError::TimeNotFound)?
        .text()
        .trim()
        .to_string())
}

pub fn parse_article(link: &str, id: i32) -> Result<Article, Error> {
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

#[derive(Debug, Fail)]
pub enum ParseHomeError {
    #[fail(display = "'.post-title a' doesn't have last element of split(' ')")]
    NoLastOfSplit,
}

pub fn parse_home_page(doc: &Document, last_id: i32) -> Result<Vec<(i32, &str)>, ParseHomeError> {
    let links = doc.find(Class("post-title").descendant(Name("a")));
    let mut result = Vec::new();

    for link in links {
        let title = link.text();
        let last = title.split(" ").last();

        match last {
            None => continue,
            Some(last_id) => {
                result.push((
                    last_id
                        .to_string()
                        .parse::<i32>()
                        .map_err(|_| ParseHomeError::NoLastOfSplit)?,
                    link.attr("href").expect("link to post should have href"),
                ));
            }
        }
    }

    Ok(result.into_iter().filter(|(x, _)| *x > last_id).collect())
}
