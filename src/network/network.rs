use super::super::add::interface;
use super::helper::HelperInterface;
use serde::Deserialize;
use std::borrow::BorrowMut;

const BASE_URL: &'static str = "https://crates.io/api/v1/crates/";

#[derive(Deserialize)]
pub struct Response {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
struct Version {
    num: String,
}

pub struct Network {
    helper: Box<dyn HelperInterface<Response>>,
}

impl Network {
    pub fn new(helper: Box<dyn HelperInterface<Response>>) -> Self {
        let network = Network { helper };
        network
    }
}

impl interface::Network for Network {
    fn get_crate_version(&self, name: String) -> Result<Vec<String>, String> {
        let url = self.helper.url(String::from(BASE_URL), name)?;
        let mut response = self.helper.response(url)?;

        if response.status().is_success() {
            let parsed_response = self.helper.parse(response.borrow_mut())?;
            let mut versions: Vec<String> = Vec::new();
            for version in parsed_response.versions {
                versions.push(version.num);
            }
            Ok(versions)
        } else if response.status().is_client_error() {
            return Err(String::from("Package not found"));
        } else {
            return Err(response.status().to_string());
        }
    }
}
