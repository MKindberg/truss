extern crate skim;
use skim::prelude::*;
use std::{fs::File, io::BufReader};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
pub struct Item {
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

    pub fn select(&self) -> Item {
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(false)
            .select1(true)
            .exit0(true)
            .preview(Some("")) // preview should be specified to enable preview window
            .build()
            .unwrap();

        let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
        self.items.iter().cloned().map(Arc::new).for_each(|x| {
            let _ = tx_item.send(x);
        });
        drop(tx_item); // so that skim could know when to stop waiting for more items.

        Skim::run_with(&options, Some(rx_item)).unwrap();
        self.items[0].clone()
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

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        self.title
            .as_ref()
            .unwrap_or(
                self.description
                    .as_ref()
                    .expect("An item must have either title or description set"),
            )
            .into()
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(
            format!(
                "{}\n{}\n{}\n{}",
                self.title.as_ref().unwrap_or(&"".to_string()),
                self.description.as_ref().unwrap_or(&"".to_string()),
                self.link.as_ref().unwrap_or(&"".to_string()),
                self.pub_date.as_ref().unwrap_or(&"".to_string())
            )
            .into(),
        )
    }
}
