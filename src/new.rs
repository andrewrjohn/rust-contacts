use console::style;
use dialoguer::Input;

use crate::{color_print, structs::ContactBook};

pub fn new_init() {
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
