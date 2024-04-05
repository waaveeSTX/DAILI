use std::{path::PathBuf, io::Read, fs};

use crate::{today::Today, paths::Paths};

fn open_file(path: &PathBuf) -> Result<fs::File, String>
{
    let file_name = path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unknown File");
    
    match fs::File::open(path)
    {
        Ok(file) => Ok(file),
        Err(_)   => Err(format!("Couldn't open {} file", file_name))
    }
}

pub struct Files
{
    pub today: fs::File,
    pub base:  fs::File
}

impl Files
{
    pub fn new(paths: &Paths) -> Result<Files, String>
    {
        let base:  fs::File = open_file(&paths.base).unwrap_or_else(|_err|
            {
                let _ = fs::File::create(&paths.base);

                open_file(&paths.base).unwrap()
            });

        let today: fs::File = open_file(&paths.today).unwrap_or_else(|_err|
            {
                let _ = fs::File::create(&paths.today);

                open_file(&paths.today).unwrap()
            });

        Ok(Files { today, base })
    }
}

pub struct Contents
{
    pub today: String,
    pub base: String
}

impl Contents
{
    pub fn new(files: &mut Files) -> Result<Contents, String>
    {
        let today: String = read_file(&mut files.today, "today.toml")?;
        let base:  String = read_file(&mut files.base,  "base.toml")?;
        
        Ok(Contents { today, base })
    }
}

fn read_file(file: &mut fs::File, file_name: &str) -> Result<String, String>
{
    let mut contents: String = String::new();

    match file.read_to_string(&mut contents)
    {
        Ok(_)        => Ok(contents),
        Err(_)       => Err(format!("Couldn't read contents for file {}", file_name))
    }
}

pub fn overwrite(who: &PathBuf, whom: &str) -> Result<(), String>
{
    fs::write(who, whom)
        .map_err(|err| err.to_string())
}

pub fn get_new_contents(today: &Today) -> Result<String, String>
{
    let new_contents = toml::to_string(today)
                            .map_err(|err| err.to_string())?;

    Ok(new_contents)
}
