use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;

/// As TOML tables that are "essential" and "optional", they need to have a blueprint so that they can later
/// be deserialized
///
/// Base files are more concise as they only store the text and ids for the tasks in the two main
/// tables
///
/// This is that table template for Base files
pub type TableTemplateForBase = BTreeMap<String, String>;

/// The Base structure containing "essential" and "optional" tables
#[derive(Deserialize, Serialize, Debug)]
pub struct Base
{
    pub essential: TableTemplateForBase,
    pub optional:  TableTemplateForBase
}

impl Base
{
    /// You use that function to create a Base object from a string, using the from_str function
    /// from toml and adding some error handling on top.
    pub fn new(base_contents: &str) -> Result<Base, String>
    {
        if base_contents.is_empty()
        {
            return Ok(Base { essential: TableTemplateForBase::new(), optional: TableTemplateForBase::new() })
        }
        
        let base: Base = toml::from_str(&base_contents)
            .map_err(|err| err.to_string())?;

        Ok(base)
    }
}
