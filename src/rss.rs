use std::{fs::File, io::BufReader};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Item {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

#[derive(Debug)]
pub struct Channel {
    title: String,
    link: String,
    description: String,
    items: Vec<Item>,
}

impl Channel {
    pub fn new(filename: &str) -> Result<Self, xml::reader::Error> {
        let file = File::open(filename)?;
        let file = BufReader::new(file);

        let mut parser = EventReader::new(file);
        let mut channel = Channel {
            title: String::new(),
            link: String::new(),
            description: String::new(),
            items: Vec::new(),
        };
        loop {
            match parser.next()? {
                XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                    "title" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            channel.title = s;
                        }
                    }
                    "link" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            channel.link = s;
                        }
                    }
                    "description" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            channel.description = s;
                        }
                    }
                    "item" => {
                        channel.items.push(Item::new(&mut parser)?);
                    }
                    _ => {}
                },
                XmlEvent::EndElement { name } => {
                    if name.local_name == "channel" {
                        return Ok(channel);
                    }
                }
                _ => {}
            }
        }
    }
}

impl Item {
    fn new(parser: &mut EventReader<BufReader<File>>) -> Result<Item, xml::reader::Error> {
        let mut item = Item {
            title: None,
            link: None,
            description: None,
            pub_date: None,
        };

        loop {
            match parser.next()? {
                XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                    "title" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            item.title = Some(s);
                        }
                    }
                    "link" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            item.link = Some(s);
                        }
                    }
                    "description" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            item.description = Some(s);
                        }
                    }
                    "pubDate" => {
                        if let Ok(XmlEvent::Characters(s)) = parser.next() {
                            item.pub_date = Some(s);
                        }
                    }
                    _ => {}
                },
                XmlEvent::EndElement { name } => {
                    if name.local_name == "item" {
                        return Ok(item);
                    }
                }
                _ => {}
            }
        }
    }
}
