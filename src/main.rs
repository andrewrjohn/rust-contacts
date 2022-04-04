mod load;
mod macros;
mod new;
mod structs;

use std::{
    fmt::{self, Display},
    process,
};

use crate::{load::load_init, new::new_init};
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)]
enum Options {
    New,
    Load,
    Quit,
}

impl Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn initial_menu() {
    println!();
    let items = vec![Options::New, Options::Load, Options::Quit];

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
        Options::Quit => process::exit(0),
    }

    initial_menu()
}

fn main() {
    initial_menu()
}
