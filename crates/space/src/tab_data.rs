use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
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
    pub favicon: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequestData {
    pub url: String,
    #[serde_as(as = "DisplayFromStr")]
    pub method: Method,
    pub params: HashMap<String, String>,
    pub body: String,
    pub authorization: String,
    pub headers: HashMap<String, String>,
}
