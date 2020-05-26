extern crate dotenv;
extern crate env_logger;
extern crate reqwest;
extern crate select;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;

use dotenv::dotenv;
use failure::Error;
use select::document::Document;

use std::env;
use std::fs;

mod bot;
mod html;
mod parsers;
mod types;

use crate::parsers::*;

const FILE_NAME: &'static str = "this_week_in_rust.last_id";

fn file_path() -> std::path::PathBuf {
    let config_dir: Option<std::path::PathBuf> = env::var("CONFIG_DIR")
        .ok()
        .and_then(|value| value.parse().ok())
        .or_else(|| dirs::config_dir());

    config_dir.unwrap_or("/tmp/".into()).join(FILE_NAME)
}

fn read_last_id() -> Result<i32, Error> {
    println!("Reading config path: {:?}", file_path().to_str());
    let src = fs::read(file_path())?;
    let src = String::from_utf8_lossy(&src);
    let value: i32 = src.trim().parse()?;

    Ok(value)
}

fn save_last_id(id: i32) -> std::io::Result<()> {
    fs::write(file_path(), id.to_string())
}

fn main() -> Result<(), Error> {
    env_logger::init();
    dotenv()?;

    let bot_token = env::var("BOT_TOKEN")
        .ok()
        .expect("Expected BOT_TOKEN env var");
    let chat_id = env::var("CHAT_ID").ok().expect("Expected CHAT_ID env var");
    let dev = env::var("DEV").ok().is_some();
    let forward_to = env::var("FORWARD_ID").ok().unwrap_or_default();
    let forward_to: Vec<&str> = forward_to
        .split(':')
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect();

    let last_id = read_last_id().unwrap_or(0);
    let mut last_id_to_be_saved = last_id;

    println!("Last ID: {}", last_id);
    println!("Starting fetch articles list...");

    let html = reqwest::get("https://this-week-in-rust.org")?.text()?;
    let document = Document::from(html.as_str());
    let links = parse_home_page(&document, last_id)?;

    if links.len() == 0 {
        println!("Nothing to send");
    } else {
        let bot = bot::Bot::new(bot_token);

        for (id, link) in links.into_iter().rev() {
            println!("\n// ——— //\nFetching #{} — {}", id, link);
            let article = parse_article(link, id)?;

            let res = bot.send_message(chat_id.clone(), article.head())?;

            if let Some(message_id) = bot::Bot::response_id(res) {
                println!("Trying forward to: {:?}", forward_to);

                for forward_to in forward_to.iter() {
                    println!("{:?}", forward_to);
                    let mut res =
                        bot.forward_message(chat_id.clone(), forward_to.to_string(), message_id)?;
                    println!("{:#?}", res.text()?);
                }
            } else {
                println!("Can't forward");
            }

            let mut _res = bot.send_message(chat_id.clone(), article.core_updates())?;

            let mut _res = bot.send_message(chat_id.clone(), article.news())?;

            let mut _res = bot.send_message(chat_id.clone(), article.crate_of_week())?;

            if id > last_id_to_be_saved {
                last_id_to_be_saved = id;
            }
        }

        if !dev {
            println!("Last ID: {}", last_id_to_be_saved);
            save_last_id(last_id_to_be_saved)?;
        }
    }

    Ok(())
}
