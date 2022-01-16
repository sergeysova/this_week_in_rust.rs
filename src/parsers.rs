use std::iter::FromIterator;

use failure::{Error, Fail};
use select::document::Document;
use select::node::Node;
use select::predicate::{And, Class, Name, Predicate};

use crate::html::escape;
use crate::types::*;

#[derive(Debug, Fail)]
pub enum ParseCrateOfWeekError {
    #[fail(display = "p not found")]
    ParagraphNotFound,

    #[fail(display = "link is not found")]
    LinkNotFound,
}

pub fn parse_crate_of_week(article: &'_ Node) -> Result<CrateOfWeek, ParseCrateOfWeekError> {
    let mut in_cow = false;

    for section in article.children() {
        match section.name().unwrap_or("") {
            "h2" => in_cow = section.text() == "Crate of the Week",
            "p" if in_cow => {
                let text = section.text();
                let link = section
                    .find(Name("a"))
                    .take(1)
                    .map(|node| {
                        (
                            node.attr("href").expect("node has no href attribute"),
                            node.text(),
                        )
                    })
                    .collect::<Vec<_>>();

                let (link, name) = link.get(0).ok_or(ParseCrateOfWeekError::LinkNotFound)?;

                return Ok(CrateOfWeek {
                    name: escape(name.to_string()),
                    text: escape(text),
                    link: escape(link.to_string()),
                });
            }
            _ => (),
        }
    }

    Err(ParseCrateOfWeekError::ParagraphNotFound)
}

#[derive(Debug, Fail)]
pub enum ParseCommunityUpdatesError {
    #[fail(display = "community updates not found")]
    NotFound,
}

fn parse_community_updates(
    article: &'_ Node,
) -> Result<CommunityUpdates, ParseCommunityUpdatesError> {
    let mut updates = Vec::new();
    let mut category = String::new();
    let mut in_community = false;

    for section in article.children() {
        match section.name().unwrap_or("") {
            "h2" => {
                in_community = section.text() == "Updates from Rust Community";
                category = String::new();
            }
            "h3" if in_community => {
                category = section.text();
            }
            "ul" if in_community && !category.is_empty() => {
                let links: Vec<Link> = links_from_html_list(&section);
                if !links.is_empty() {
                    updates.push(NamedLinksList {
                        name: category,
                        links: LinksList::from_iter(links.into_iter()),
                    });
                }
                category = String::new();
            }
            _ => (),
        }
    }

    Ok(CommunityUpdates { updates })
}

fn links_from_html_list(node: &'_ Node) -> Vec<Link> {
    node.descendants()
        .map(Link::from_node)
        .filter_map(Result::ok)
        .collect()
}

#[derive(Debug, Fail)]
pub enum ParseUpdatesError {
    #[fail(display = "updates from rust project not found")]
    NotFound,
}

pub fn parse_updates_from_core(article: &'_ Node) -> Result<Vec<Link>, ParseUpdatesError> {
    let mut in_core = false;
    for section in article.children() {
        match section.name().unwrap_or("") {
            "h2" => {
                in_core = section.text() == "Updates from the Rust Project";
            }
            "ul" if in_core => {
                let links: Vec<Link> = links_from_html_list(&section);
                if !links.is_empty() {
                    return Ok(links);
                }
            }
            _ => (),
        }
    }

    Err(ParseUpdatesError::NotFound)
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

    really_parse_article(&document, link, id)
}

fn really_parse_article(document: &Document, link: &str, id: i32) -> Result<Article, Error> {
    let article = document
        .find(And(Name("article"), Class("post-content")))
        .next()
        .ok_or(ParseCommunityUpdatesError::NotFound)?;

    let date = parse_article_date(document)?;
    let community = parse_community_updates(&article)?;
    let crate_of_week = parse_crate_of_week(&article)?;
    let core = CoreUpdates::new(parse_updates_from_core(&article)?);

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
        let last = title.split(' ').last();

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
