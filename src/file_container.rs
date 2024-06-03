use std::{fs::{self, OpenOptions}, path::PathBuf};

pub struct FileContainer
{
    pub object:           fs::File,
    pub path:             PathBuf,
    pub initial_contents: String
}

impl FileContainer
{
    pub fn new(path_to_file: &PathBuf) -> Result<FileContainer, String>
    {
        let file:  fs::File = OpenOptions::new()
                                          .read(true)
                                          .write(true)
                                          .create(true)
                                          .open(&path_to_file).map_err(|err| err.to_string())?;

        let contents = fs::read_to_string(&path_to_file).map_err(|err| err.to_string())?;

        Ok(FileContainer { object: file, path: path_to_file.to_path_buf(), initial_contents: contents })
    }
}
