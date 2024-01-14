mod rss;

fn main() -> std::io::Result<()> {
    let channel = rss::Channel::new("sample-rss-2.xml").expect("Failed to parse xml");

    dbg!(channel);
    Ok(())
}
