use std::{borrow::BorrowMut, fmt::Display};

use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde_json::json;

use crate::{
    color_print, initial_menu,
    structs::{Address, Contact, ContactBook},
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

#[derive(Debug)]
enum YesNo {
    Yes,
    No,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn add(book: &mut ContactBook) {
    let name: String = Input::new().with_prompt("Name").interact_text().unwrap();
    let phone: String = Input::new().with_prompt("Phone").interact_text().unwrap();

    let add_address_items = vec![YesNo::Yes, YesNo::No];

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Add physical address?")
        .items(&add_address_items)
        .report(true)
        .default(1)
        .interact()
        .unwrap();

    let contact = Contact {
        name,
        phone_number: phone.parse().expect("phone number must be a number"),
        address: Address::from_empty(),
    };

    match add_address_items[index] {
        YesNo::Yes => todo!(),
        YesNo::No => {
            let _ = book.add_contact(contact).expect("can't add contact");
        }
    }
}

fn load_menu(name: String) {
    println!();
    match ContactBook::from_disk(name.as_str()) {
        Some(mut contacts) => {
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
                LoadMenu::Add => {
                    add(contacts.borrow_mut());
                    load_menu(name)
                }
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
