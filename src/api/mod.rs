/// Interact with the Asana API

mod users;

use reqwest::blocking::{Client};
use serde::de::DeserializeOwned;

use crate::schema;
use crate::BASE_URL;

type AccessToken = String;

/// This handles interactions with the Asana API.
///
/// It posts payloads to the API and returns the result. It also handles
/// authentication through a Personal Access Token (PAT)
pub struct API {
    client: Client,
    pat: AccessToken,
    url: String
}

impl API {
    /// Creates a new API struct from the given token
    pub fn from_token<S: AsRef<str>>(token: S) -> Self {
        API {
            client: Client::new(),
            pat: String::from(token.as_ref()),
            url: String::from(BASE_URL)
        }
    }

    /// Returns the token provided when the API struct was created
    pub fn token(&self) -> &str {
        &self.pat
    }

    /// Adds a url segment to the url
    pub fn request<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
        self.url += url.as_ref();
        self
    }

    /// Executes the request
    pub fn get(&mut self) -> Result<schema::Response, Box<dyn std::error::Error>> {
        let resp = self.client
            .get(&self.url)
            .bearer_auth(&self.pat)
            .send()?;

        let text = resp.text()?;
        let resp = serde_json::from_str::<schema::Response>(&text)?;

        // Reset the url
        self.url = format!("{}", BASE_URL);
        return Ok(resp);
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
        let mut asana = API::from_token(get_pat());
        let resp = asana.request( users::me() ).get().expect("Couldn't perform request");
        let user = resp.value::<schema::User>();
        assert!(user.is_some());
        assert_eq!(user.unwrap().resource_type, "user");
    }
}
