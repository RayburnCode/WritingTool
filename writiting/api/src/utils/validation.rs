// src/utils/validation.rs
use lazy_static::lazy_static;
use regex::Regex;

pub const URL_REGEX_STR: &str = r"^https?://.*";

lazy_static! {
    pub static ref URL_REGEX: Regex = Regex::new(URL_REGEX_STR).unwrap();
}

pub fn validate_url(url: &str) -> bool {
    URL_REGEX.is_match(url)
    
}

pub fn validate_http_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
