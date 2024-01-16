use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    filename: Option<String>,
    #[arg(short, long)]
    xml_file: Option<String>,
}
