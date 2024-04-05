use colored::Colorize;
use home::home_dir;
use std::path::PathBuf;
use std::fs;
use crate::warning;

fn get_home_dir() -> Result<PathBuf, String>
{
    home_dir().ok_or_else(|| "Couldn't find home path".to_string())
}

fn get_daili_path() -> Result<PathBuf, String>
{
    let mut daili: PathBuf = get_home_dir()?;
    daili.push(".daili");

    match daili.exists()
    {
        true  => Ok(daili),
        false =>
        {
            warning::print_warning("The .daili directory doesn't exist yet");
            println!("{}", "Creating one for you...".blue());

            if let Err(_) = fs::create_dir(&daili)
            {
                return Err("Failed to create the daili directory".to_string());
            }

            Ok(daili)
        }
    }
}

pub struct Paths
{
    pub daili: PathBuf,
    pub today: PathBuf,
    pub base:  PathBuf,
}

impl Paths
{
    pub fn new() -> Result<Paths, String>
    {
        let daili: PathBuf = get_daili_path()?;
        let today: PathBuf = daili.join("today.toml");
        let base:  PathBuf = daili.join("base.toml");

        Ok(Paths { daili, today, base })
    }
}
