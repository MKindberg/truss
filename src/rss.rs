extern crate skim;
use skim::prelude::*;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
pub struct Item {
    title: Option<String>,
    pub link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Channel {
    title: String,
    link: String,
    description: String,
    pub items: Vec<Item>,
}

impl Channel {
    pub fn new<R: std::io::Read>(reader: BufReader<R>) -> Result<Self, xml::reader::Error> {
        let mut parser = EventReader::new(reader);
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
                        let i = Item::new(&mut parser)?;
                        if i.title.is_none() && i.description.is_none() {
                            continue;
                        }
                        channel.items.push(i);
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
    fn new<R: std::io::Read>(
        parser: &mut EventReader<BufReader<R>>,
    ) -> Result<Item, xml::reader::Error> {
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
    pub fn open(&self) {
        if self.link.is_none() {
            return;
        }
        let _ = std::process::Command::new("xdg-open")
            .arg(self.link.as_ref().unwrap())
            .output()
            .expect("failed to open link in browser");
    }
}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        self.title
            .as_ref()
            .unwrap_or_else(|| {
                self.description
                    .as_ref()
                    .expect("An item must have either title or description set")
            })
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
impl SkimItem for Channel {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.title)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(
            format!("{}\n{}\n{}", self.title, self.description, self.link,).into(),
        )
    }
}
