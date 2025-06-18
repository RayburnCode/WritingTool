use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

pub const URL_REGEX_STR: &str = r"^https?://.*";

lazy_static! {
    pub static ref URL_REGEX: Regex = Regex::new(URL_REGEX_STR).unwrap();
}


pub fn validate_http_url(url: &str) -> Result<(), ValidationError> {
    if URL_REGEX.is_match(url) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid HTTP URL"))
    }
}