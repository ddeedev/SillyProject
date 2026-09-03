use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TabData {
    Browser(BrowserData),
    ApiRequest(ApiRequestData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserData {
    pub url: String,
    pub tab_number: Option<u8>,
    pub favicon: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequestData {
    pub url: String,
    pub tab_number: Option<u8>,
    // for api request favicon will ba base on its method
    pub favicon: String,
    #[serde_as(as = "DisplayFromStr")]
    pub method: Method,
    pub params: HashMap<String, String>,
    pub body: String,
    pub authorization: String,
    pub headers: HashMap<String, String>,
}

impl BrowserData {
    pub fn new(url: String, favicon: String) -> Self {
        Self {
            url,
            favicon,
            tab_number: None,
        }
    }
}

impl TabData {
    pub fn new_tab(url: String, favicon: String) -> Self {
        Self::Browser(BrowserData::new(url, favicon))
    }
}
