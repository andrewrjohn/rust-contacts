mod macros;
mod structs;

use crate::structs::ContactBook;
use clap::Parser;
use console::style;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, help = "Create a new contact book")]
    new: Option<String>,

    #[clap(long, help = "View a contact book by name")]
    view: Option<String>,
}

fn new_command(name: String) {
    match ContactBook::new(name.as_str()) {
        Ok(created) => {
            color_print!(green, "Created contact book: {}", created.name)
        }
        Err(err) => color_print!(red, "Error creating contact book: {}", err),
    }
}

fn main() {
    let args = Args::parse();

    if let Some(name) = args.new {
        new_command(name)
    }

    if let Some(name) = args.view {
        let book = ContactBook::from_disk(name.as_str());
        println!("{}", book)
    }
}
