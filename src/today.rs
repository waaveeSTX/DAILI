use crate::{serde::{Serialize, Deserialize}, task::UnidentifiedTask, extracting, base::Base, files::{self, Contents}, Colorize};
use std::path::PathBuf;
use chrono::prelude::*;
use std::collections::BTreeMap;

pub type TableTemplate = BTreeMap<String, UnidentifiedTask>;

#[derive(Deserialize, Serialize)]
pub struct Today
{
    pub date: String,
    pub essential: TableTemplate,
    pub optional:  TableTemplate
}

impl Today
{
    fn get_contents_from_base(base: &Base, current_date: &str) -> Result<String, String>
    {
        let new_today = extracting::convert_base_to_today(base, current_date);
        let new_today_contents: String = toml::to_string(&new_today)
                                            .map_err(|err| err.to_string())?;

        Ok(new_today_contents)
    }

    pub fn handle(contents: &Contents, today_path: &PathBuf, base: &Base) -> Result<Today, String>
    {
        let current_date = Utc::now().date_naive().format("%Y-%m-%d").to_string();

        let mut today_contents: String = contents.today.clone();
        let mut should_rewrite = today_contents.is_empty();

        let mut today: Today;

        loop
        {
            if should_rewrite
            {
                println!("{}", "rewriting...\n".blue());

                today_contents = Today::get_contents_from_base(base, current_date.as_str())?;
                files::overwrite(today_path, &today_contents)?;
            }

            today = toml::from_str(&today_contents)
                .map_err(|err| err.to_string())?;

            match !should_rewrite && today.date != current_date
            {
                true  => should_rewrite = true,
                false => break
            };
        }

        Ok(today)
    }
}
