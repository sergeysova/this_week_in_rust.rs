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
pub enum ParseCommunityUpdatesError {
    #[fail(display = "#news-blog-posts not found")]
    IdNotFound,

    #[fail(display = "next element not found")]
    NextNotFound,
}

fn parse_community_updates_subtopic(
    updates: &Document,
    topic: &str,
) -> Result<LinksList, ParseCommunityUpdatesError> {
    Ok(updates
        .find(Attr("id", topic))
        .next()
        .ok_or(ParseCommunityUpdatesError::IdNotFound)?
        .next()
        .ok_or(ParseCommunityUpdatesError::NextNotFound)?
        .next()
        .ok_or(ParseCommunityUpdatesError::NextNotFound)?
        .find(Name("ul").descendant(Name("li")))
        .map(Link::from_node)
        .filter(Result::is_ok)
        .map(|v| v.expect("Here should be only Ok"))
        .collect())
}

// Topics list is taken from https://github.com/rust-lang/this-week-in-rust/blob/master/draft/2021-02-03-this-week-in-rust.md
pub fn parse_updates_from_community(doc: &Document) -> Result<CommunityUpdates, ParseCommunityUpdatesError> {
    // Check that community updates section exists
    doc.find(Attr("id", "updates-from-rust-community"))
        .next()
        .ok_or(ParseCommunityUpdatesError::IdNotFound)?;

    // Unfortunately TWiR sometimes skips some topics in the atricle, so best that can be done error-handling wise is to ignore the missing ones
    let collect_subtopic =
        |topic| parse_community_updates_subtopic(&doc, topic).unwrap_or_default();

    Ok(CommunityUpdates {
        official: collect_subtopic("official"),
        newsletters: collect_subtopic("newsletters"),
        tooling: collect_subtopic("projecttooling-updates"),
        observations: collect_subtopic("observationsthoughts"),
        walkthoughs: collect_subtopic("rust-walkthroughs"),
        misc: collect_subtopic("miscellaneous"),
    })
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
    let community = parse_updates_from_community(&document)?;
    let crate_of_week = parse_crate_of_week(&document)?;
    let core = CoreUpdates::new(parse_updates_from_core(&document)?);

    Ok(Article {
        id,
        link: link.to_string(),
        date,
        community,
        crate_of_week,
        core,
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
