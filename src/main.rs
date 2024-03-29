mod args;
mod picker;
mod rss;

use clap::Parser;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    let channels = if let Some(filename) = args.filename {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        parse_channels(reader.lines())
    } else if let Some(xml_file) = args.xml_file {
        let file = File::open(xml_file)?;
        let reader = BufReader::new(file);
        let channel = rss::Channel::new(reader).expect("Failed to parse xml");
        vec![channel]
    } else {
        parse_channels(std::io::stdin().lock().lines())
    };
    let channel = picker::select(&channels);
    let item = picker::select(&channel.items);

    item.open();
    Ok(())
}

fn parse_channels<B: BufRead>(lines: std::io::Lines<B>) -> Vec<rss::Channel> {
    let mut channels = Vec::new();
    for url in lines {
        let resp = reqwest::blocking::get(url.unwrap()).unwrap();
        let file = BufReader::new(resp);
        let channel = rss::Channel::new(file).expect("Failed to parse xml");
        if channel.items.is_empty() {
            continue;
        }
        channels.push(channel);
    }
    channels
}
