use failure::Fail;
use regex::Regex;
use std::{fmt, iter::FromIterator};

use crate::select;
use crate::select::predicate::Name;

use crate::html::escape;

#[derive(Debug)]
pub struct Link {
    link: String,
    text: String,
}

#[derive(Debug, Fail)]
pub enum LinkFromNodeError {
    #[fail(display = "next element not found")]
    NextNotFound,
    #[fail(display = "[href] nof found")]
    HrefNotFound,
}

impl Link {
    pub fn from_node<'a>(node: select::node::Node<'a>) -> Result<Link, LinkFromNodeError> {
        let text = node.text().replace(". [discuss]", "");
        let link = node
            .find(Name("a"))
            .next()
            .ok_or(LinkFromNodeError::NextNotFound)?
            .attr("href")
            .ok_or(LinkFromNodeError::HrefNotFound)?;

        Ok(Link {
            link: escape(link.to_string()),
            text: escape(text),
        })
    }
}

fn link_to_github(link: &String) -> String {
    let re = Regex::new(r"https?://github.com/(.*)$").unwrap();
    re.replace_all(&link, "$1").to_string()
}

fn link_as_clear(link: &String) -> String {
    let re = Regex::new(r"/.+$").unwrap();
    let preshort = link
        .replace("https://", "")
        .replace("http://", "")
        .replace(".html", "")
        .replace(".htm", "");
    re.replace_all(preshort.as_ref(), "").to_string()
}

fn link_to_medium(link: &String) -> String {
    let re = Regex::new(r"https?://medium.com/(.+)/.+").unwrap();
    re.replace_all(&link, "medium.com/$1").to_string()
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let link_text = if self.link.contains("github.com") {
            link_to_github(&self.link)
        } else if self.link.contains("medium.com") {
            link_to_medium(&self.link)
        } else {
            link_as_clear(&self.link)
        };

        write!(
            f,
            "{text}\n<a href=\"{link}\">{link_text}</a>\n",
            link = self.link,
            text = self.text,
            link_text = link_text
        )
    }
}

#[derive(Debug, Default)]
pub struct LinksList(Vec<Link>);

impl LinksList {
    fn fmt_with_subtitle(&self, f: &mut fmt::Formatter, subtitle: &str) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(f, "\n<b>{}</b>\n", subtitle)?;
        write!(f, "{}", self)
    }
}

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

impl FromIterator<Link> for LinksList {
    fn from_iter<T: IntoIterator<Item = Link>>(iter: T) -> Self {
        LinksList(iter.into_iter().collect())
    }
}

#[derive(Debug)]
pub struct CommunityUpdates {
    pub official: LinksList,
    pub newsletters: LinksList,
    pub tooling: LinksList,
    pub observations: LinksList,
    pub walkthoughs: LinksList,
    pub misc: LinksList,
}

impl fmt::Display for CommunityUpdates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<b>Updates from Rust Community</b>\n")?;
        self.official.fmt_with_subtitle(f, "Official")?;
        self.newsletters.fmt_with_subtitle(f, "Newsletters")?;
        self.tooling
            .fmt_with_subtitle(f, "Project/Tooling Updates")?;
        self.observations
            .fmt_with_subtitle(f, "Observations/Thoughts")?;
        self.walkthoughs.fmt_with_subtitle(f, "Rust Walkthroughs")?;
        self.misc.fmt_with_subtitle(f, "Miscellaneous")
    }
}

#[derive(Debug)]
pub struct CrateOfWeek {
    pub name: String,
    pub text: String,
    pub link: String,
}

impl fmt::Display for CrateOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<b>Crate of the week:</b> <a href=\"{link}\">{name}</a>\n\n{text}\n",
            link = self.link,
            name = self.name,
            text = self.text
        )
    }
}

#[derive(Debug)]
pub struct CoreUpdates(LinksList);

impl CoreUpdates {
    pub fn new(list: Vec<Link>) -> Self {
        CoreUpdates(LinksList(list))
    }
}

impl fmt::Display for CoreUpdates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<b>Updates from core</b>\n\n{}", self.0)
    }
}

#[derive(Debug)]
pub struct Article {
    pub(super) id: i32,
    pub(super) date: String,
    pub(super) link: String,
    pub(super) community: CommunityUpdates,
    pub(super) crate_of_week: CrateOfWeek,
    pub(super) core: CoreUpdates,
}

impl Article {
    pub fn head(&self) -> String {
        format!(
            "<b>This week in Rust #{id}</b> — {date}\n\n{link}",
            id = self.id,
            link = self.link,
            date = self.date.to_lowercase()
        )
    }

    pub fn community_updates(&self) -> String {
        format!("{}", self.community)
    }

    pub fn crate_of_week(&self) -> String {
        format!("{}", self.crate_of_week)
    }

    pub fn core_updates(&self) -> String {
        format!("{}", self.core)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_fmt() {
        let link = Link {
            link: "FOO".to_string(),
            text: "BAR".to_string(),
        };

        assert_eq!(format!("{}", link), "BAR\nFOO\n".to_string());
    }

    #[test]
    fn links_fmt() {
        let links = LinksList(vec![
            Link {
                link: "linkA".to_string(),
                text: "textA".to_string(),
            },
            Link {
                link: "linkB".to_string(),
                text: "textB".to_string(),
            },
            Link {
                link: "linkC".to_string(),
                text: "textC".to_string(),
            },
        ]);

        let expected = "textA\nlinkA\n\ntextB\nlinkB\n\ntextC\nlinkC\n".to_string();

        assert_eq!(format!("{}", links), expected);
    }
}
