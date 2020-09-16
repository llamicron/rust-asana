extern crate serde;
extern crate serde_json;
extern crate reqwest;


pub mod api;
pub mod resources;

// Dev helper
// This gets the personal access token from .token in the crate root
fn get_pat() -> String {
    let token = std::fs::read_to_string(".token").unwrap();
    token.trim().to_string()
}
