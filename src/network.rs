use super::add::interface;
use reqwest;
use serde::Deserialize;

const BASE_URL: &'static str = "https://crates.io/api/v1/crates/";

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "crate")]
    crate_: Crate,
}

#[derive(Deserialize)]
struct Crate {
    max_version: String,
}

pub struct Network {}

impl interface::Network for Network {
    fn get_crate_version(&self, name: String) -> Result<String, String> {
        let url = create_url(name)?;
        let mut response = create_response(url)?;

        if response.status().is_success() {
            let version = get_response_max_version(&mut response)?;
            return Ok(version);
        } else if response.status().is_client_error() {
            return Err(String::from("Package not found"));
        } else {
            return Err(response.status().to_string());
        }
    }
}

fn create_url(name: String) -> Result<reqwest::Url, String> {
    match reqwest::Url::parse(&(String::from(BASE_URL) + &name)) {
        Err(e) => return Err(e.to_string()),
        Ok(url) => return Ok(url),
    };
}

fn create_response(url: reqwest::Url) -> Result<reqwest::Response, String> {
    match reqwest::get(url) {
        Err(e) => return Err(e.to_string()),
        Ok(resp) => return Ok(resp),
    }
}

fn get_response_max_version(resp: &mut reqwest::Response) -> Result<String, String> {
    match resp.json::<Response>() {
        Err(e) => return Err(e.to_string()),
        Ok(result) => return Ok((result as Response).crate_.max_version),
    }
}
