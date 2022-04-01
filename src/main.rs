mod macros;
mod structs;

use crate::structs::{Contact, ContactBook};
use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use serde_json::json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, help = "Create a new contact book")]
    new: Option<String>,

    #[clap(long, help = "View a contact book by name")]
    view: Option<String>,
}

fn new() {
    let input: String = Input::new()
        .with_prompt("Give your contact book a name")
        .interact_text()
        .unwrap();
    match ContactBook::new(input.as_str()) {
        Ok(created) => {
            color_print!(green, "Created contact book: {}", created.name)
        }
        Err(err) => color_print!(red, "Error creating contact book: {}", err),
    }
}

#[derive(Debug)]
enum SingleOptions {
    Search,
    Add,
    Remove,
}

impl std::fmt::Display for SingleOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn search(contacts: &ContactBook) {
    let search_input: String = Input::new()
        .with_prompt("Enter the name of the contact book")
        .interact_text()
        .unwrap();

    let searched: Vec<Contact> = contacts
        .contacts
        .into_iter()
        .filter(|c| {
            c.name
                .to_lowercase()
                .contains(search_input.to_lowercase().as_str())
        })
        .collect();

    let obj = json!(searched);

    println!();
    color_print!(cyan, "Search results ({})", searched.len());
    println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    search(contacts);
}

fn load() {
    let input: String = Input::new()
        .with_prompt("Enter the name of the contact book")
        .interact_text()
        .unwrap();

    match ContactBook::from_disk(input.as_str()) {
        Some(contacts) => {
            println!(
                "Viewing: {} ({} contacts found)",
                contacts.name,
                contacts.contacts.len()
            );

            search(&contacts);
        }
        None => color_print!(red, "Contact book not found: {}", input),
    }
}

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

fn input_loop() {
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
        Options::New => new(),
        Options::Load => load(),
    }
    input_loop()
}

fn main() {
    input_loop()
}
