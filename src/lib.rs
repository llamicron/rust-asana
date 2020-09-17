#![allow(dead_code, unused_variables, unused)]
extern crate serde;
extern crate serde_json;
extern crate reqwest;


pub mod api;
pub mod schema;

// Dev helper
// This gets the personal access token from .token in the crate root
#[cfg(test)]
fn get_pat() -> String {
    let token = std::fs::read_to_string(".token").expect(".token file probably isn't present");
    token.trim().to_string()
}
