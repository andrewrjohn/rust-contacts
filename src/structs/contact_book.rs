use std::{fs, path::Path};

use super::Contact;
use console::style;
use serde::{Deserialize, Serialize};

use crate::color_print;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactBook {
    pub name: String,
    pub contacts: Vec<Contact>,
}

impl std::fmt::Display for ContactBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl ContactBook {
    pub fn new(name: &str) -> Result<Self, &'static str> {
        color_print!(cyan, "Creating...");
        let book = Self {
            name: name.to_string(),
            contacts: vec![],
        };

        return match Path::exists(Path::new(&book.path_from_name())) {
            true => Err("Contact book with that name already exists."),
            false => {
                let _ = book.save();
                Ok(book)
            }
        };
    }

    fn path_from_name(&self) -> String {
        format!("{}.json", self.name)
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(&self).unwrap();

        fs::write(format!("{}.json", self.name), serialized)
    }

    pub fn from_disk(name: &str) -> Self {
        let raw = fs::read_to_string(format!("{}.json", name)).unwrap();
        let contact_book: ContactBook = serde_json::from_str(raw.as_str()).unwrap();

        contact_book
    }

    pub fn add_contact(&mut self, contact: Contact) -> Result<Contact, &'static str> {
        if self.contacts.contains(&contact) {
            return Err("contact already exists");
        }

        self.contacts.push(contact.clone());

        return match self.save() {
            Ok(_) => Ok(contact.clone()),
            Err(_) => Err("can't save contact to disk"),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::Address;

    use super::*;

    fn create_mock_contact() -> Contact {
        let address = Address {
            street: "123 Main St.".to_string(),
            city: "Pleasantville".to_string(),
            state: "Tennessee".to_string(),
            postal_code: "55555".to_string(),
        };

        Contact {
            name: String::from("Andrew"),
            phone_number: 7632480172,
            address,
        }
    }

    #[test]
    fn add_contact() {
        let contact = create_mock_contact();
        let mut contact_book = ContactBook::new("test1").unwrap();
        let added = contact_book
            .add_contact(contact.clone())
            .expect("contact to be added");

        assert_eq!(added, contact);
    }

    #[test]
    #[should_panic]
    fn add_duplicate_contact() {
        let contact = create_mock_contact();
        let contact1 = create_mock_contact();

        let mut contact_book = ContactBook::new("test").unwrap();

        contact_book
            .add_contact(contact)
            .expect("contact to be added");

        contact_book
            .add_contact(contact1)
            .expect("contact to be added");
    }
}
