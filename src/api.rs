/// Interact with the Asana API

use reqwest::blocking::{Client};
use crate::schema::{AsanaResponse, AsanaResponseWrapper};

type AccessToken = String;

pub const BASE_URL: &'static str = "https://app.asana.com/api/1.0";

/// This handles interactions with the Asana API.
/// 
/// It posts payloads to the API and returns the result. It also handles
/// authentication through a Personal Access Token (PAT)
pub struct API {
    client: Client,
    pat: AccessToken
}

impl API {
    /// Creates a new API struct from the given token
    pub fn from_token<S: AsRef<str>>(token: S) -> Self {
        API {
            client: Client::new(),
            pat: String::from(token.as_ref())
        }
    }

    /// Returns the token provided when the API struct was created
    pub fn token(&self) -> &str {
        &self.pat
    }

    /// Just makes a get request with the PAT and returns the result
    pub fn get<S: AsRef<str>>(&self, url: S) -> Result<AsanaResponse, reqwest::Error> {
        let resp = self.client
            .get(url.as_ref())
            .bearer_auth(&self.pat)
            .send()?;
        
        // This isn't working :(
        let text = resp.text()?;
        let asana_resp = serde_json::from_str::<AsanaResponseWrapper>(&text).expect("Couldn't get asana resp");
        Ok(asana_resp.data)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_pat;

    #[test]
    fn new_api_with_token() {
        let api = API::from_token("my token");
        assert_eq!(api.token(), "my token");
    }

    #[test]
    fn test_get_me() {
        let client = API::from_token(get_pat());
        let url = format!("{}/users/me", BASE_URL);
        let resp = client.get(&url).expect("Couldn't perform request");
        match resp {
            AsanaResponse::User(user) => {
                assert!(user.name.len() > 1);
            },
            _ => assert!(false)
        }
    }
}
