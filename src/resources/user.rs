/// An Asana User
/// 
/// https://developers.asana.com/docs/users

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize)]
struct User {
    gid: usize,
    resource_type: String,
    name: String,
    email: String,
    photo: HashMap<String, String>,
    // TODO: Replace Value with Workspace once it's done
    workspaces: Vec<Value>
}
