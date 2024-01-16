mod args;
mod rss;

use clap::Parser;

fn main() -> std::io::Result<()> {
    let _args = args::Args::parse();
    let channel = rss::Channel::new("sample-rss-2.xml").expect("Failed to parse xml");
    // dbg!(channel);
    channel.select();
    Ok(())
}
