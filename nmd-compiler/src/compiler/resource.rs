mod locatable;          // TODO: future use
mod location;           // TODO: future use

use std::{path::PathBuf, str::FromStr, fs::{File, self, OpenOptions}, io::{self, Write}};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ResourceError {

    #[error("resource '{0}' not found")]
    ResourceNotFound(String),

    #[error("resource is invalid")]
    InvalidResource,

    #[error("resource is invalid because: {0}")]
    InvalidResourceVerbose(String),

    #[error("resource cannot be created: {0}")]
    Creation(String),

    #[error("resource '{0}' cannot be read")]
    ReadError(String),

    #[error(transparent)]
    IoError(#[from] io::Error)
}

#[derive(Debug, Clone)]
pub struct Resource {
    name: String, 
    location: PathBuf       // TODO: migrate to structured type to handle URL
}

impl FromStr for Resource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(ResourceError::Creation("resource cannot be an empty string".to_string()));
        }

        Ok(Resource::new(PathBuf::from_str(s).unwrap())?)
    }
}


impl ToString for Resource {
    fn to_string(&self) -> String {
        self.location().to_string_lossy().to_string()
    }
}

impl Resource {

    pub fn location(&self) -> &PathBuf {
        &self.location
    } 

    pub fn new(location: PathBuf) -> Result<Self, ResourceError> {
        match location.file_name() {
            Some(name) => Ok(Self {
                name: name.to_string_lossy().to_string(),
                location
            }),
            None => Err(ResourceError::InvalidResource)
        }

        
    }

    pub fn content(&self) -> Result<String, ResourceError> {

        match fs::read_to_string(self.location.clone()) {           // TODO: remove clone
            Ok(content) => Ok(content),
            Err(err) => Err(ResourceError::ReadError(format!("error during read content of {}: {}", self.to_string(), err.to_string())))
        }
    } 

    pub fn name(&self) -> &String {
        &self.name        
    }

    pub fn write(&self, content: &str) -> Result<(), ResourceError> {
        let file_path = &self.location;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn append(&self, content: &str) -> Result<(), ResourceError> {
        let file_path = &self.location;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn from_str() {

        let path = ".";

        let resource = Resource::from_str(path);
    
        match resource {
            Ok(location) => assert_eq!(location.to_string(), path),
            Err(e) => panic!("'{}' during location generation from str of path: '{}'", e, path)
        }
    }
}
