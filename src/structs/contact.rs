use serde::{Deserialize, Serialize};

use super::Address;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone_number: i64,
    pub address: Address,
}
