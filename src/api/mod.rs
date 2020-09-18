/// Interact with the Asana API
mod users;

use url::Url;

use reqwest::blocking::{Client};
use serde::de::DeserializeOwned;

use crate::schema;
use crate::{BASE_URL, BASE_API};

type AccessToken = String;


/// This handles interactions with the Asana API.
///
/// It posts payloads to the API and returns the result. It also handles
/// authentication through a Personal Access Token (PAT)
pub struct API {
    client: Client,
    pat: AccessToken,
    url: Url
}

impl API {
    /// Creates a new API struct from the given token
    pub fn from_token<S: AsRef<str>>(token: S) -> Self {
        API {
            client: Client::new(),
            pat: String::from(token.as_ref()),
            url: Url::parse(BASE_URL).unwrap()
        }
    }

    fn query(&mut self, name: &str, value: &str) -> &mut Self {
        self.url.query_pairs_mut().append_pair(name, value);
        self
    }

    /// Returns the token provided when the API struct was created
    pub fn token(&self) -> &str {
        &self.pat
    }

    /// Sets the path of the url
    pub fn request<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
        let uri = format!("{}{}", BASE_API, url.as_ref());
        self.url = self.url.join(&uri).expect(
            &format!("Not a valid url path: {}", url.as_ref())
        );
        self
    }


    /// Executes the request
    pub fn get(&mut self) -> Result<schema::Response, Box<dyn std::error::Error>> {
        let resp = self.client
            .get(self.url.as_str())
            .bearer_auth(&self.pat)
            .send()?;

        let text = resp.text()?;
        let resp = serde_json::from_str::<schema::Response>(&text)?;

        // Reset the url
        return Ok(resp);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_pat;
    use url::Url;

    #[test]
    fn new_api_with_token() {
        let api = API::from_token("my token");
        assert_eq!(api.token(), "my token");
    }
}
