use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde_json::json;

use crate::{
    color_print, initial_menu,
    structs::{Contact, ContactBook},
};

#[derive(Debug)]
enum LoadMenu {
    Search,
    Add,
    Remove,
    Back,
}

impl std::fmt::Display for LoadMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn search(contacts: ContactBook) {
    println!();
    let search_input: String = Input::new()
        .with_prompt("Search by name (or 'quit' to cancel search)")
        .interact_text()
        .unwrap();

    if search_input != "quit" {
        let searched: Vec<Contact> = contacts
            .contacts
            .clone()
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
}

fn load_menu(name: String) {
    match ContactBook::from_disk(name.as_str()) {
        Some(contacts) => {
            color_print!(
                cyan,
                "Viewing: {} ({} contacts found)",
                contacts.name,
                contacts.contacts.len()
            );

            let items = vec![
                LoadMenu::Search,
                LoadMenu::Add,
                LoadMenu::Remove,
                LoadMenu::Back,
            ];

            let selected_index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Menu")
                .report(true)
                .default(0)
                .items(&items)
                .interact()
                .unwrap();

            match items[selected_index] {
                LoadMenu::Search => {
                    search(contacts);
                    load_menu(name)
                }
                LoadMenu::Add => todo!(),
                LoadMenu::Remove => todo!(),
                LoadMenu::Back => initial_menu(),
            }
        }
        None => color_print!(red, "Contact book not found: {}", name),
    }
}

pub fn load_init() {
    let input: String = Input::new()
        .with_prompt("Enter the name of the contact book")
        .with_initial_text("my_contacts")
        .interact_text()
        .unwrap();

    load_menu(input);
}
