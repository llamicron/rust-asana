#![allow(unused)]

pub mod response;
pub use response::Response;

/// https://developers.asana.com/docs/schemas
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct AsanaNamedResource {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct AsanaResource {
    pub gid: String,
    pub resource_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Error {
    pub help: Option<String>,
    pub message: Option<String>,
    pub phrase: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Errors {
    pub errors: Vec<Error>,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
    pub email_domains: Option<Vec<String>>,
    pub is_organization: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub image_128x128: String,
    pub image_21x21: String,
    pub image_27x27: String,
    pub image_36x36: String,
    pub image_60x60: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
    pub email: String,
    pub photo: Option<Photo>,
    pub workspaces: Vec<Workspace>,
}

#[derive(Deserialize, Debug)]
pub struct UserCompact {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asana_named_resource() {
        let raw = r#"{
            "gid": "12345",
            "resource_type": "task",
            "name": "Bug Task"
        }"#;

        let resource: AsanaNamedResource = serde_json::from_str(raw).unwrap();
        assert_eq!(resource.name, "Bug Task");
        assert_eq!(resource.resource_type, "task");
    }

    #[test]
    fn test_asana_resource() {
        let raw = r#"{
            "gid": "12345",
            "resource_type": "task"
        }"#;

        let resource: AsanaResource = serde_json::from_str(raw).unwrap();
        assert_eq!(resource.resource_type, "task");
        assert_eq!(resource.gid, "12345");
    }

    #[test]
    fn test_workspace() {
        let raw = r#"{
            "gid": "12345",
            "resource_type": "workspace",
            "name": "My Company Workspace",
            "email_domains": [
              "asana.com"
            ],
            "is_organization": false
          }"#;

        let ws: Workspace = serde_json::from_str(raw).unwrap();
        assert_eq!(ws.resource_type, "workspace");
        assert_eq!(ws.gid, "12345");
        assert_eq!(ws.email_domains.unwrap().len(), 1);
        assert!(!ws.is_organization.unwrap());
    }

    #[test]
    fn test_user() {
        let raw = r#"{
            "gid": "12345",
            "resource_type": "user",
            "name": "Greg Sanchez",
            "email": "gsanchez@example.com",
            "photo": {
              "image_128x128": "https://...",
              "image_21x21": "https://...",
              "image_27x27": "https://...",
              "image_36x36": "https://...",
              "image_60x60": "https://..."
            },
            "workspaces": [
              {
                "gid": "12345",
                "resource_type": "workspace",
                "name": "My Company Workspace"
              }
            ]
          }"#;

        let user: User = serde_json::from_str(raw).unwrap();
        assert_eq!(user.name, "Greg Sanchez");
        assert_eq!(user.gid, "12345");
        assert_eq!(user.photo.unwrap().image_128x128, "https://...");
        assert_eq!(user.workspaces[0].resource_type, "workspace");
    }

    #[test]
    fn test_error() {
        let raw = r#"{
            "errors": [
              {
                "help": "For more information on API status codes and how to handle them, read the docs on errors: https://asana.github.io/developer-docs/#errors'",
                "message": "project: Missing input",
                "phrase": "6 sad squid snuggle softly"
              }
            ]
          }"#;

        let errors: Errors = serde_json::from_str(raw).unwrap();
        assert_eq!(errors.errors.len(), 1);
        assert!(errors.errors[0].help.is_some());
        assert!(errors.errors[0].message.is_some());
        assert!(errors.errors[0].phrase.is_some());
    }
}
