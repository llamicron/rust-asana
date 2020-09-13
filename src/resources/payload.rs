/// A generic payload

use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize, Debug)]
struct VectorPayload {
    data: Vec<Value>
}

#[derive(Deserialize, Debug)]
struct SinglePayload {
    data: Value
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_single_payload() {
        let payload_str = r#"{
            "data": {
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
            }
          }"#;

        let payload = serde_json::from_str::<SinglePayload>(payload_str);
        assert!(payload.is_ok());
    }

    #[test]
    fn test_deserialize_vector_payload() {
        let payload_str = r#"{
            "data": [
              {
                "gid": "12345",
                "resource_type": "task",
                "name": "Bug Task"
              }
            ]
          }"#;

        let payload = serde_json::from_str::<VectorPayload>(payload_str);
        assert!(payload.is_ok());
        assert!(payload.unwrap().data.len() == 1);
    }
}
