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
use eframe::{
    egui::{self, popup, Area, Context, RichText, TextEdit, Visuals},
    epaint::{pos2, Color32, Stroke, Vec2},
    epi::{App, Frame},
    NativeOptions,
};
use structs::{Address, Contact, ContactBook};

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

struct ContactsApp {
    selected: bool,
    search_text: String,
    show_add_contact_window: bool,
    new_name: String,
    new_number: String,
}

impl Default for ContactsApp {
    fn default() -> Self {
        Self {
            selected: false,
            search_text: String::new(),
            show_add_contact_window: false,
            new_name: String::new(),
            new_number: String::new(),
        }
    }
}

const SM: f32 = 4.0;
const MD: f32 = 8.0;
const LG: f32 = 12.0;
const XL: f32 = 20.0;

impl App for ContactsApp {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        // ctx.set_visuals(Visuals::light());

        let mut book = ContactBook::from_disk("my_contacts").unwrap();

        egui::SidePanel::left("Menu")
            .width_range(50.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(SM);
                    ui.heading("Contacts");

                    ui.add_space(SM);
                    ui.add(
                        TextEdit::singleline(&mut self.search_text)
                            .hint_text("Search for a contact book")
                            .margin(Vec2 { x: 20.0, y: 10.0 }),
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Popup window for new contact
            egui::Window::new("Add New Contact")
                .open(&mut self.show_add_contact_window)
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.set_min_width(200.0);
                    ui.add_space(SM);

                    ui.add(
                        TextEdit::singleline(&mut self.new_name)
                            .hint_text("Name")
                            .margin(Vec2 { x: 20.0, y: 10.0 }),
                    );
                    ui.add(
                        TextEdit::singleline(&mut self.new_number)
                            .hint_text("Phone #")
                            .margin(Vec2 { x: 20.0, y: 10.0 }),
                    );
                    ui.add_space(LG);
                    if ui
                        .button(RichText::new("Save").color(Color32::GREEN))
                        .clicked()
                    {
                        if let Ok(added) = book.add_contact(Contact {
                            name: self.new_name.clone(),
                            phone_number: self
                                .new_number
                                .clone()
                                .parse::<i64>()
                                .expect("Must be number"),
                            address: Address::from_empty(),
                        }) {
                            self.new_name = String::new();
                            self.new_number = String::new();

                            // self.show_add_contact_window = false;
                        }
                    }
                });

            ui.heading(book.name);
            ui.add_space(SM);
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new("Add").color(Color32::GREEN))
                    .clicked()
                {
                    self.show_add_contact_window = !self.show_add_contact_window
                }
            });

            ui.add_space(LG);
            for contact in book.contacts {
                ui.add_space(MD);
                ui.label(contact.name);
                ui.label(format!("Phone: {}", contact.phone_number));
            }
        });
    }

    fn name(&self) -> &str {
        "Contacts"
    }
}

fn main() {
    // initial_menu()

    let app = ContactsApp::default();
    let native_options = NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
