use crate::{base::Base, extracting, files::Contents, serde::{Deserialize, Serialize}, task::UnidentifiedTask, Colorize};
use std::{fs, path::PathBuf};
use std::collections::BTreeMap;

// As explained in base.rs, this is the table template but for Today files, that require more
// information about the task (the completion status)
pub type TableTemplate = BTreeMap<String, UnidentifiedTask>;

// Today, the object that is created from the Today file (~/.daili/today.toml) and displayed
#[derive(Deserialize, Serialize)]
pub struct Today
{
    pub date:      String,
    pub essential: TableTemplate,
    pub optional:  TableTemplate
}

impl Today
{
    // This function handles the state of wether the contents for today should be rewritten with
    // the ones from base or not, checking if the day had passed (achieving that by checking if the
    // date written in the today.toml file is the same as the current) and if the contents for today
    // are empty, which happens when you configure your base and then run the program.
    pub fn handle(contents: &Contents, today_path: &PathBuf, base: &Base, current_date: &str) -> Result<Today, String>
    {
        let mut today_contents: String = contents.today.clone();
        let mut should_rewrite = today_contents.is_empty();

        let mut today: Today;

        loop
        {
            if should_rewrite
            {
                println!("{}", "rewriting...\n".blue());

                let new_today: Today = extracting::convert_base_to_today(base, current_date);
                today_contents = toml::to_string(&new_today).map_err(|err| err.to_string())?;
                fs::write(today_path, &today_contents).map_err(|err| err.to_string())?;
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
