use std::{fs::{self, OpenOptions}, io::Read};

use crate::paths::Paths;

// Files needed
pub struct Files
{
    pub today: fs::File,
    pub base:  fs::File
}

impl Files
{
    // Open needed files one by one using the paths for them
    pub fn new(paths: &Paths) -> Result<Files, String>
    {
        let base:  fs::File = OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(&paths.base).map_err(|err| err.to_string())?;

        let today: fs::File = OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(&paths.today).map_err(|err| err.to_string())?;

        Ok(Files { today, base })
    }
}

// Contents from the open files
pub struct Contents
{
    pub today: String,
    pub base: String
}

impl Contents
{
    // From the open files, get their contents one by one
    pub fn new(files: &mut Files) -> Result<Contents, String>
    {
        let base:  String = read_file(&mut files.base,  "base.toml")?;
        let today: String = read_file(&mut files.today, "today.toml")?;
        
        Ok(Contents { today, base })
    }
}

pub fn read_file(file: &mut fs::File, file_name: &str) -> Result<String, String>
{
    let mut contents: String = String::new();

    match file.read_to_string(&mut contents)
    {
        Ok(_)        => Ok(contents),
        Err(_)       => Err(format!("Couldn't read contents for file {}", file_name))
    }
}
