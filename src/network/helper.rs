use serde::Deserialize;

pub trait HelperInterface<T> {
    fn url(&self, base_url: String, endpoint: String) -> Result<reqwest::Url, String>;
    fn response(&self, url: reqwest::Url) -> Result<reqwest::Response, String>;
    fn parse(&self, response: &mut reqwest::Response) -> Result<T, String>
    where
        for<'de> T: Deserialize<'de>;
}

pub struct Helper {}

impl<T> HelperInterface<T> for Helper {
    fn url(&self, base_url: String, endpoint: String) -> Result<reqwest::Url, String> {
        let mut url: String = base_url.clone();
        url.push_str(&endpoint);
        match reqwest::Url::parse(&url) {
            Err(e) => return Err(e.to_string()),
            Ok(url) => return Ok(url),
        }
    }

    fn response(&self, url: reqwest::Url) -> Result<reqwest::Response, String> {
        match reqwest::get(url) {
            Err(e) => return Err(e.to_string()),
            Ok(url) => return Ok(url),
        }
    }

    fn parse(&self, response: &mut reqwest::Response) -> Result<T, String>
    where
        for<'de> T: Deserialize<'de>,
    {
        match response.json::<T>() {
            Err(e) => return Err(e.to_string()),
            Ok(result) => return Ok(result),
        }
    }
}
