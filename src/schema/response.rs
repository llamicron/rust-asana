use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

use crate::schema;

#[derive(Deserialize)]
pub struct Response {
    #[serde(default)]
    pub data: serde_json::Value,
    #[serde(default)]
    pub errors: Vec<schema::Error>,
}

impl Response {
    pub fn into<T: DeserializeOwned>(self) -> Result<T, Vec<schema::Error>> {
        if self.errors.len() > 0 {
            return Err(self.errors);
        }

        match serde_json::from_value::<T>(self.data) {
            Ok(value) => Ok(value),
            Err(_) => Err(vec![])
        }
    }

    pub fn value<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_value::<T>(self.data.clone()).ok()
    }

    pub fn errors(&self) -> Option<Vec<schema::Error>> {
        if self.errors.len() > 0 {
            return Some(self.errors.clone());
        } else {
            return None;
        }
    }

    pub fn values<T: DeserializeOwned>(&self) -> Option<Vec<T>> {
        serde_json::from_value::<Vec<T>>(self.data.clone()).ok()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn test_value_resp() -> &'static str {
        return r#"{
            "data": {
                "gid": "12345",
                "resource_type": "user",
                "name": "Greg Sanchez"
            }
        }"#;
    }

    fn test_vector_resp() -> &'static str {
        return r#"{
            "data": [
                {
                    "gid": "12345",
                    "resource_type": "user",
                    "name": "Greg Sanchez"
                },
                {
                    "gid": "54321",
                    "resource_type": "user",
                    "name": "Luke Lastname"
                }
            ]
        }"#;
    }

    fn test_error_resp() -> &'static str {
        return r#"{
            "errors": [
                {
                    "help": "For more information on API status codes and how to handle them, read the docs on errors: https://asana.github.io/developer-docs/#errors'",
                    "message": "project: Missing input",
                    "phrase": "6 sad squid snuggle softly"
                }
            ]
        }"#;
    }

    #[test]
    fn test_basic_deserialize_value() {
        let payload = test_value_resp();
        let resp = serde_json::from_str::<Response>(payload);
        assert!(resp.is_ok());
    }

    #[test]
    fn test_basic_deserialize_vector() {
        let payload = test_vector_resp();
        let resp = serde_json::from_str::<Response>(payload);
        assert!(resp.is_ok());
    }

    #[test]
    fn test_basic_deserialize_errors() {
        let payload = test_error_resp();
        let resp = serde_json::from_str::<Response>(payload);
        assert!(resp.is_ok());
    }

    #[test]
    fn test_deserialize_to_resource() {
        let payload = test_value_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        let resource = resp.into::<schema::UserCompact>().unwrap();
        assert_eq!(resource.name, "Greg Sanchez");
        assert_eq!(resource.gid, "12345");
    }

    #[test]
    fn test_deserialize_to_vector_resources() {
        let payload = test_vector_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        let resources = resp.into::<Vec<schema::UserCompact>>().unwrap();
        assert_eq!(resources.len(), 2);
        assert_eq!(resources[0].name, "Greg Sanchez");
        assert_eq!(resources[0].gid, "12345");
    }

    #[test]
    fn test_deserialize_asana_errors() {
        let payload = test_error_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        let errors = resp.into::<schema::UserCompact>().unwrap_err();
        assert_eq!(errors.len(), 1);
        let first_error = errors[0].clone();
        assert_eq!(first_error.message.unwrap(), "project: Missing input");
    }

    #[test]
    fn test_get_value() {
        let payload = test_value_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        if let Some(user) = resp.value::<schema::UserCompact>() {
            assert_eq!(user.name, "Greg Sanchez");
        } else {
            assert!(false);
        }
    }
}
