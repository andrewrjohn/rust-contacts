mod structs;

use crate::structs::{Address, Contact, ContactBook};

fn main() {
    // let mut contact_book = ContactBook::new("business_contacts");

    // let address = Address {
    //     street: "123 Main St.".to_string(),
    //     city: "Pleasantville".to_string(),
    //     state: "Tennessee".to_string(),
    //     postal_code: "55555".to_string(),
    // };

    // let contact = Contact {
    //     name: String::from("Andrew"),
    //     phone_number: 7632480172,
    //     address,
    // };

    // contact_book
    //     .add_contact(contact)
    //     .expect("contact to be added");

    let contact_book_disk = ContactBook::from_disk("business_contacts");

    println!("{:?}", contact_book_disk);
}
