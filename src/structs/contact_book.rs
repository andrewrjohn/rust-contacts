use std::{fs, path::Path};

use super::Contact;
use console::style;
use serde::{Deserialize, Serialize};

use crate::color_print;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

        return match Path::exists(Path::new(&book.get_path())) {
            true => Err("Contact book with that name already exists."),
            false => {
                let _ = book.save();
                Ok(book)
            }
        };
    }

    fn get_path(&self) -> String {
        format!("{}.json", self.name)
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(&self).unwrap();

        fs::write(format!("{}.json", self.name), serialized)
    }

    pub fn from_disk(name: &str) -> Option<ContactBook> {
        if let Ok(raw) = fs::read_to_string(format!("{}.json", name)) {
            let contact_book: ContactBook = serde_json::from_str(raw.as_str()).unwrap();

            Some(contact_book)
        } else {
            None
        }
    }

    pub fn delete(&self) {
        fs::remove_file(self.get_path()).unwrap();
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
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

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

    fn get_rand_string() -> String {
        let s: String = thread_rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        s
    }

    #[test]
    fn new_method() {
        let contact_book = ContactBook::new(get_rand_string().as_str()).unwrap();
        assert!(Path::exists(Path::new(&contact_book.get_path())));
        contact_book.delete();
    }

    #[test]
    fn delete_method() {
        let contact_book = ContactBook::new(get_rand_string().as_str()).unwrap();
        contact_book.delete();
        assert!(!Path::exists(Path::new(&contact_book.get_path())));
    }

    #[test]
    fn add_contact_method() {
        let contact = create_mock_contact();
        let mut contact_book = ContactBook::new(get_rand_string().as_str()).unwrap();
        let added = contact_book
            .add_contact(contact.clone())
            .expect("contact to be added");

        assert_eq!(added, contact);

        contact_book.delete();
    }

    #[test]
    #[should_panic]
    fn add_contact_method_with_duplicate() {
        let contact = create_mock_contact();
        let contact1 = create_mock_contact();

        let mut contact_book = ContactBook::new(get_rand_string().as_str()).unwrap();
        contact_book.delete();

        contact_book
            .add_contact(contact)
            .expect("contact to be added");

        contact_book
            .add_contact(contact1)
            .expect("contact to be added");
    }

    #[test]
    fn get_path_method() {
        let name = get_rand_string();
        let contact_book = ContactBook::new(name.as_str()).unwrap();
        assert_eq!(contact_book.get_path(), format!("{}.json", name.as_str()));
        contact_book.delete();
    }
}
