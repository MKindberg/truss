mod args;
mod rss;

use std::{fs::File, io::BufReader};
use clap::Parser;

fn main() -> std::io::Result<()> {
    let _args = args::Args::parse();
    let file = File::open("sample-rss-2.xml")?;
    let file = BufReader::new(file);
    let resp = reqwest::blocking::get("https://www.rssboard.org/files/sample-rss-2.xml").unwrap();
    let file = BufReader::new(resp);
    let channel = rss::Channel::new(file).expect("Failed to parse xml");
    channel.select();
    Ok(())
}
