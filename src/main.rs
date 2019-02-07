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
use select::document::Document;

use std::env;
use std::error::Error;
use std::fs;

mod bot;
mod html;
mod parsers;
mod types;

use crate::parsers::*;

const FILE_NAME: &'static str = "this_week_in_rust.last_id";

fn file_path() -> std::path::PathBuf {
    dirs::config_dir().unwrap_or("/tmp/".into()).join(FILE_NAME)
}

fn read_last_id() -> Result<i32, Box<dyn Error + 'static>> {
    let src = fs::read(file_path())?;
    let src = String::from_utf8_lossy(&src);
    let value: i32 = src.trim().parse()?;

    Ok(value)
}

fn save_last_id(id: i32) -> std::io::Result<()> {
    fs::write(file_path(), id.to_string())
}

fn run() -> Result<(), Box<dyn Error>> {
    let bot_token = env::var("BOT_TOKEN")
        .ok()
        .expect("Expected BOT_TOKEN env var");
    let chat_id = env::var("CHAT_ID").ok().expect("Expected CHAT_ID env var");
    let forward_to = env::var("FORWARD_ID")
        .ok()
        .expect("Expected FORWARD_ID env var");
    let forward_to = forward_to.split(':').fold(Vec::new(), |mut v, s| {
        v.push(s);
        v
    });

    let last_id = read_last_id().unwrap_or(0);
    let mut last_id_to_be_saved = last_id;

    println!("Last ID: {}", last_id);
    println!("Starting fetch articles list...");
    let html = reqwest::get("https://this-week-in-rust.org")?.text()?;
    let document = Document::from(html.as_str());
    let links = parse_home_page(&document, last_id);

    if links.len() == 0 {
        println!("Nothing to send");
    } else {
        let bot = bot::Bot::new(bot_token);

        for (id, link) in links.into_iter().rev() {
            println!("\n// ——— //\nFetching #{} — {}", id, link);
            let article = parse_article(link, id)?;

            let res = bot.send_message(chat_id.clone(), article.head())?;

            if let Some(message_id) = bot::Bot::response_id(res) {
                for forward_to in forward_to.iter() {
                    let _ =
                        bot.forward_message(chat_id.clone(), forward_to.to_string(), message_id)?;
                }
            }

            let mut _res = bot.send_message(chat_id.clone(), article.core_updates())?;

            let mut _res = bot.send_message(chat_id.clone(), article.news())?;

            let mut _res = bot.send_message(chat_id.clone(), article.crate_of_week())?;

            if id > last_id_to_be_saved {
                last_id_to_be_saved = id;
            }
        }

        save_last_id(last_id_to_be_saved)?;
    }

    println!("Last ID: {}", last_id_to_be_saved);

    Ok(())
}

fn main() {
    env_logger::init();
    dotenv().expect("Failed to run dotenv");

    run().unwrap();
}
