use crate::serde::Deserialize;
use std::collections::BTreeMap;

pub type TableTemplateForBase = BTreeMap<String, String>;

#[derive(Deserialize)]
pub struct Base
{
    pub essential: TableTemplateForBase,
    pub optional:  TableTemplateForBase
}

impl Base
{
    pub fn from_str(base_contents: &str) -> Result<Base, String>
    {
        if base_contents.is_empty()
        {
            return Err("You don't have a base.toml file".to_string());
        }

        toml::from_str(base_contents)
            .map_err(|err| err.to_string())
    }
}
