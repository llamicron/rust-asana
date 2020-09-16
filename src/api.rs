/// Interact with the Asana API

use reqwest::blocking::{Client, Response};

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
    pub fn get<S: AsRef<str>>(&self, url: S) -> Result<Response, reqwest::Error> {
        self.client
            .get(url.as_ref())
            .bearer_auth(&self.pat)
            .send()
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
    fn test_get_request() {
        let api = API::from_token("token");
        let resp = api.get("https://postman-echo.com/get/").unwrap();
        assert_eq!(resp.status(), 200);
        assert!(resp.text().unwrap().len() > 10);
    }

    #[test]
    fn test_pat_required() {
        let bad_api = API::from_token("invalid_token");
        // get_pat() gets my token from the environment
        // Only for testing, token should be passed into
        // API::from_token()
        let good_api = API::from_token(get_pat());
        
        let url = "https://app.asana.com/api/1.0/users/me";

        let bad_resp = bad_api.get(url).unwrap();
        let good_resp = good_api.get(url).unwrap();

        assert!(bad_resp.text().unwrap().contains("errors"));
        assert!(good_resp.text().unwrap().contains("email"));

    }
}
