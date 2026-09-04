use http::{Method, request::Parts};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, Map, serde_as};
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

impl ApiRequestData {
    pub fn new(
        url: String,
        method: Method,
        params: HashMap<String, String>,
        body: String,
        auth: String,
        headers: HashMap<String, String>,
    ) -> Self {
        let mut favicon: String = String::new();
        match method {
            Method::POST => {
                favicon.push_str("RED");
            }
            Method::GET => {
                favicon.push_str("GREEN");
            }
            Method::PUT => {
                favicon.push_str("CYAN");
            }
            Method::DELETE => {
                favicon.push_str("DARK");
            }
            Method::PATCH => {
                favicon.push_str("YELLOW");
            }
            _ => favicon.push_str("WHITE"),
        };
        Self {
            url,
            authorization: auth,
            body,
            headers,
            params,
            favicon,
            method,
            tab_number: None,
        }
    }
}

impl TabData {
    pub fn new_browser_tab(url: String, favicon: String) -> Self {
        Self::Browser(BrowserData::new(url, favicon))
    }

    pub fn new_api_tab(
        url: String,
        method: Method,
        params: HashMap<String, String>,
        body: String,
        auth: String,
        headers: HashMap<String, String>,
    ) -> Self {
        Self::ApiRequest(ApiRequestData::new(
            url, method, params, body, auth, headers,
        ))
    }
}
