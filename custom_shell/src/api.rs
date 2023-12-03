use reqwest::blocking::Client;
use reqwest::blocking::Response;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct CustomApi {
    name: String,
    owner_id: String,
    api_url: String,
    pub username: String,
    pub response: String,
}

impl CustomApi {
    pub fn new(name: &str, owner_id: &str, api_url: &str) -> Self {
        Self {
            name: name.to_string(),
            owner_id: owner_id.to_string(),
            api_url: api_url.to_string(),
            username: String::new(),
            response: String::new(),
        }
    }
    pub fn init(&mut self) -> anyhow::Result<()> {
        let mut req_data: Vec<(&str, &str)> = Vec::new();
        req_data.push(("app_name", &self.name));
        req_data.push(("owner_id", &self.owner_id));

        let response = Self::request(req_data, &self.api_url);
        let text = response.text().unwrap();
        if text == "Success" {
            Result::Ok(())
        } else {
            Result::Err()
        }
        Ok(())
    }

    fn request(req_data: Vec<(&str, &str)>, url: &str) -> Response {
        let client = Client::new();
        let mut req_data_str = String::new();
        for d in req_data {
            req_data_str.push_str(&format!("{}/{}/", d.0, d.1))
        }
        req_data_str = req_data_str.strip_suffix("/").unwrap().to_string();
        let req = format!("{url}/{req_data_str}");

        client
            .post(req)
            .header("User-Agent", "CustomAuth")
            // .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .unwrap()
    }
}
