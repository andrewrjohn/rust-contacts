use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}, {}\n{}",
            self.street, self.city, self.state, self.postal_code
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_string() {
        let address = Address {
            street: "123 Main St.".to_string(),
            city: "Pleasantville".to_string(),
            state: "Tennessee".to_string(),
            postal_code: "55555".to_string(),
        };

        assert_eq!(
            format!("{}", address),
            "123 Main St.\nPleasantville, Tennessee\n55555"
        );
    }
}
