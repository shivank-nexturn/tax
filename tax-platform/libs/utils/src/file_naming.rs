use regex::Regex;
use anyhow::Result;

pub struct FileNamingValidator {
    tax_file_regex: Regex,
    currency_file_regex: Regex,
    geo_file_regex: Regex,
}

impl FileNamingValidator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            tax_file_regex: Regex::new(r"^tax_\d{8}_\d{6}\.zip$")?,
            currency_file_regex: Regex::new(r"^curr_\d{8}_\d{6}\.zip$")?,
            geo_file_regex: Regex::new(r"^geo_\d{8}_\d{6}\.zip$")?,
        })
    }

    pub fn validate_tax_file(&self, filename: &str) -> bool {
        self.tax_file_regex.is_match(filename)
    }

    pub fn validate_currency_file(&self, filename: &str) -> bool {
        self.currency_file_regex.is_match(filename)
    }

    pub fn validate_geo_file(&self, filename: &str) -> bool {
        self.geo_file_regex.is_match(filename)
    }
}
