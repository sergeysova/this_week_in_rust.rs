extern crate env_logger;
extern crate reqwest;
extern crate select;

use select::document::Document;

use std::error::Error;
use std::fs;
use std::io::Write;

mod parsers;
mod types;

use parsers::*;

const SAVE_PATH: &'static str = "/tmp/this_week_in_rust_last_id.txt";

fn read_last_id() -> Result<i32, Box<dyn Error + 'static>> {
    let src = fs::read(SAVE_PATH)?;
    let src = String::from_utf8_lossy(&src);
    let value: i32 = src.trim().parse()?;

    Ok(value)
}

fn save_last_id(id: i32) -> Result<(), Box<dyn Error + 'static>> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(SAVE_PATH)?;

    file.write_all(id.to_string().as_bytes())?;

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
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
        for (ref id, link) in links.iter().rev() {
            println!("\n// ——— //\nFetching #{} — {}", id, link);
            let res = parse_article(link, *id)?;

            println!("{}", res);

            if *id > last_id_to_be_saved {
                last_id_to_be_saved = *id;
            }
        }

        save_last_id(last_id_to_be_saved)?;
    }

    println!("Last ID: {}", last_id_to_be_saved);

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}
