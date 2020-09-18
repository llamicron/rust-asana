use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

use crate::schema;

/// Represents a response from Asana
///
/// This is a JSON object that could have 2 (or more idk) keys: "data" or "errors". You can
/// deserialize a payload from the Asana API into one of these.
///
/// You can use `value()` or `values()` to get the data returned by Asana, serialized into one
/// of the structs in the [`schema`](crate::schema) module. You can use `errors()` to return a
/// vector of `schema::Error`s.
#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(default)]
    pub data: serde_json::Value,
    #[serde(default)]
    pub errors: Vec<schema::Error>,
}

impl Response {
    /// Consumes the response, returning `Ok()` with the type provided or
    /// `Err()` with a vector of `schema::Error`s.
    pub fn into<T: DeserializeOwned>(self) -> Result<T, Vec<schema::Error>> {
        if self.errors.len() > 0 {
            return Err(self.errors);
        }

        match serde_json::from_value::<T>(self.data) {
            Ok(value) => Ok(value),
            Err(_) => Err(vec![])
        }
    }

    /// Returns the single value of type T. If Asana returns an array of objects,
    /// this will return None. See `values()`.
    ///
    /// ```rust
    /// use rust_asana::Response;
    /// use rust_asana::schema::UserCompact;
    ///
    /// // This is UserCompact object, defined by Asana
    /// let payload = r#"{
    ///     "data": {
    ///         "gid": "12345",
    ///         "resource_type": "user",
    ///         "name": "Greg Sanchez"
    ///     }
    /// }"#;
    ///
    /// let resp = serde_json::from_str::<Response>(&payload).unwrap();
    /// let user = resp.value::<UserCompact>();
    /// assert!(user.is_some());
    /// assert_eq!(user.unwrap().name, "Greg Sanchez");
    /// ```
    pub fn value<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_value::<T>(self.data.clone()).ok()
    }

    /// Same as `values()`, but returns the Asana errors (always a vector).
    pub fn errors(&self) -> Option<Vec<schema::Error>> {
        if self.errors.len() > 0 {
            return Some(self.errors.clone());
        } else {
            return None;
        }
    }

    /// This is the same as `value()`, but it returns a vector of values. Some requests
    /// from Asana return an array of values, which this is meant for.
    ///
    /// **Important Note**: This methid *will work* on a payload of only one object. It
    /// will convert it to a vector of one item. As long as there is serializable data in the
    /// `data` field of the payload, this will return the object(s).
    ///
    /// If the response is an empty array, this will return Some([]), not None
    ///
    /// ```rust
    /// use rust_asana::Response;
    /// use rust_asana::schema::UserCompact;
    ///
    /// // This is UserCompact object, defined by Asana
    /// let payload = r#"{
    ///     "data": [
    ///         {
    ///             "gid": "12345",
    ///             "resource_type": "user",
    ///             "name": "Greg Sanchez"
    ///         },
    ///         {
    ///             "gid": "54321",
    ///             "resource_type": "user",
    ///             "name": "Luke Lastname"
    ///         }
    ///     ]
    /// }"#;
    ///
    /// let resp = serde_json::from_str::<Response>(&payload).unwrap();
    /// let user = resp.values::<UserCompact>();
    /// assert!(user.is_some());
    /// assert_eq!(user.unwrap()[0].name, "Greg Sanchez");
    /// ```
    pub fn values<T: DeserializeOwned>(&self) -> Option<Vec<T>> {
        if let Some(value) = self.value::<T>() {
            Some(vec![value])
        } else {
            serde_json::from_value::<Vec<T>>(self.data.clone()).ok()
        }
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
        assert!(resp.errors().is_none());
    }

    #[test]
    fn test_get_vector_values() {
        let payload = test_vector_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        if let Some(users) = resp.values::<schema::UserCompact>() {
            assert_eq!(users.len(), 2);
            assert_eq!(users[0].name, "Greg Sanchez");
        } else {
            assert!(false);
        }
        assert!(resp.errors().is_none());
    }

    #[test]
    fn test_get_errors() {
        let payload = test_error_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        assert!(resp.values::<schema::UserCompact>().is_none());
        if let Some(errors) = resp.errors() {
            assert_eq!(errors.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_value_to_values() {
        // This returns a single value, not a vector
        let payload = test_value_resp();
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        // I want resp.values() to convert to a vector of one item
        if let Some(users) = resp.values::<schema::UserCompact>() {
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].name, "Greg Sanchez");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_empty_return_vector() {
        let payload = r#"{ "data": [] }"#;
        let resp = serde_json::from_str::<Response>(payload).unwrap();
        let items = resp.values::<schema::UserCompact>();
        assert!(items.is_some());
        assert_eq!(items.unwrap().len(), 0);
    }
}
