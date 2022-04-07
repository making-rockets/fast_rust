use std::path::Path;
use config_struct::StructOptions;

pub struct ParseToml {}

impl ParseToml {
    #[test]
    pub fn parse_toml(toml: &Path) -> anyhow::Result<()> {
        let result = config_struct::create_struct("../setting.toml", "src/config/toml_config.rs", &StructOptions::default());
        Ok(())
    }


}

#[test]
fn test(){
    let result = ParseToml::parse_toml(&Path::new(""));
}