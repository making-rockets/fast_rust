use std::path::Path;
use toml_edit::Document;

pub struct ParseToml {}

impl ParseToml {
    pub fn parse_toml(toml: &Path) {
        let file_str = std::fs::read_to_string(toml).expect("this toml read failed");
        let _result = file_str.parse::<Document>().expect("toml parse failed");
    }
}
