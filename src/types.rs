use std::fmt;

use select;
use select::predicate::Name;

#[derive(Debug)]
pub struct Link {
    link: String,
    text: String,
}

impl Link {
    pub fn from_node<'a>(node: select::node::Node<'a>) -> Link {
        let text = node.text().replace(". [discuss]", "");
        let link = node.find(Name("a")).next().unwrap().attr("href").unwrap();
        Link {
            link: link.to_string(),
            text: text,
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<a href=\"{}\">{}</a>", self.link, self.text)
    }
}

#[derive(Debug)]
pub struct LinksList(Vec<Link>);

impl fmt::Display for LinksList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stringified = self
            .0
            .iter()
            .map(|link| format!("{}", link))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", stringified)
    }
}

#[derive(Debug)]
pub struct News(LinksList);

impl News {
    pub fn new(list: Vec<Link>) -> Self {
        News(LinksList(list))
    }
}

impl fmt::Display for News {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<b>News</b>\n{}", self.0)
    }
}

#[derive(Debug)]
pub struct CrateOfWeek {
    pub(super) name: String,
    pub(super) text: String,
    pub(super) link: String,
}

impl fmt::Display for CrateOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<b>Crate of the week:</b> <a href=\"{link}\">{name}</a>\n{text}\n",
            link = self.link,
            name = self.name,
            text = self.text
        )
    }
}

#[derive(Debug)]
pub struct Updates(LinksList);

impl Updates {
    pub fn new(list: Vec<Link>) -> Self {
        Updates(LinksList(list))
    }
}

impl fmt::Display for Updates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<b>Updates from core</b>\n{}", self.0)
    }
}

#[derive(Debug)]
pub struct Article {
    pub(super) id: i32,
    pub(super) date: String,
    pub(super) link: String,
    pub(super) news: News,
    pub(super) crate_of_week: CrateOfWeek,
    pub(super) updates: Updates,
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<b>#{id}</b> — <a href={link}>{date}</a>\n\n{news}\n\n{crate}\n\n{updates}",
            id = self.id,
            link = self.link,
            date = self.date,
            news = self.news,
            crate = self.crate_of_week,
            updates = self.updates,
        )
    }
}
