use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub filename: Option<String>,
    #[arg(short, long)]
    pub xml_file: Option<String>,
}
