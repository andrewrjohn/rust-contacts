mod load;
mod macros;
mod new;
mod structs;

use crate::{load::load_init, new::new_init};
// use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     #[clap(long, help = "Create a new contact book")]
//     new: Option<String>,

//     #[clap(long, help = "View a contact book by name")]
//     view: Option<String>,
// }

#[derive(Debug)]
enum Options {
    New,
    Load,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn initial_menu() {
    println!();
    let items = vec![Options::New, Options::Load];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Menu:")
        .report(true)
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    match items[selection] {
        Options::New => new_init(),
        Options::Load => load_init(),
    }

    initial_menu()
}

fn main() {
    initial_menu()
}
