mod args;
mod rss;
mod picker;

use clap::Parser;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    if let Some(filename) = args.filename {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut channels = Vec::new();
        for url in reader.lines() {
            let resp = reqwest::blocking::get(url.unwrap()).unwrap();
            let file = BufReader::new(resp);
            let channel = rss::Channel::new(file).expect("Failed to parse xml");
            channels.push(channel);
        }
        let channel = picker::select(&channels);
        let _item = picker::select(&channel.items);
    }
    Ok(())
}
