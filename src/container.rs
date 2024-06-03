use std::path::PathBuf;
use crate::{base::Base, today::Today, file_container::FileContainer};

/// A struct with the Containerizable trait is a desserialized object that in the proccess of its
/// creation requires all the information needed to use a file into the
/// context of the program, and so it can be containerized for better organization
pub trait Containerizable
{
    /// Creates an object of any type that implements this trait
    /// That is, only the common parameters between their constructors' mustn't be optional
    fn from_contents(contents: &str, path: &PathBuf, base_object: Option<&Base>, current_date: Option<&str>) -> Result<Self, String>
    where 
        Self: Sized;
}

// As base doesn't need as many parameters in its constructor, the from_contents function in this
// case uses only the contents for the construction.
impl Containerizable for Base 
{
    fn from_contents(contents: &str, _path: &PathBuf, _base_object: Option<&Base>, _current_date: Option<&str>) -> Result<Self, String>
    where 
        Self: Sized
    {
        Base::new(contents)
    }
}

// In this case we check if all the parameters are provided and then use them all to construct
// today.
impl Containerizable for Today 
{
    fn from_contents(contents: &str, path: &PathBuf, base_object: Option<&Base>, current_date: Option<&str>) -> Result<Self, String>
    where 
        Self: Sized
    {
        if let (Some(base_object), Some(current_date)) = (base_object, current_date)
        {
            Today::handle(contents, path, base_object, current_date)
        }
        else
        {
            Err("Base object or current date is missing".to_string())
        }
    }
}

/// The encapsulation of the file container and the object extracted from it
pub struct Container<Any: Containerizable>
{
    pub file:   FileContainer,
    pub object: Any
}

impl<Any: Containerizable> Container<Any>
{
    /// Use the information needed for the initialization of a Container Object
    pub fn init(path: &PathBuf, base_object: Option<&Base>, current_date: Option<&str>) -> Result<Container<Any>, String>
    {
        let file = match FileContainer::new(path)
        {
            Ok(base_file) => base_file,
            Err(err) => return Err(err)
        };

        let object = match Any::from_contents(&file.initial_contents, path, base_object, current_date)
        {
            Ok(base) => base,
            Err(err) => return Err(err)
        };

        Ok(Container { file, object })
    }
}
