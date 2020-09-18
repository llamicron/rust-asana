#![allow(dead_code, unused_variables, unused)]
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate url;


pub mod api;
pub mod schema;

pub use schema::Response;
pub use api::API;

pub const BASE_URL: &'static str = "https://app.asana.com";
pub const BASE_API: &'static str = "/api/1.0";

// Dev helper
// This gets the personal access token from .token in the crate root
#[cfg(test)]
fn get_pat() -> String {
    let token = std::fs::read_to_string(".token").expect(".token file probably isn't present");
    token.trim().to_string()
}
