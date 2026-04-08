pub mod base;
pub mod crypter;
pub mod errors;
pub mod table;
pub mod user;

pub fn approved(string: &str) -> bool {
    !string.is_empty() && string.is_ascii() && string.len() <= 20
}
