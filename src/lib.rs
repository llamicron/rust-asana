extern crate serde;
extern crate serde_json;
extern crate reqwest;


pub mod api;
pub mod error;

// pub type Result<T> = std::result::Result<T, error::Error>;

// Dev helper
// This gets the personal access token from .token in the crate root
fn get_pat() -> String {
    let token = std::fs::read_to_string(".token").unwrap();
    token.trim().to_string()
}
