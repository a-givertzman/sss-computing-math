use std::{io::BufReader, fs::File, path::Path};
use serde::Deserialize;
use log::warn;

pub struct JsonFile { }

///
/// Reads json file
impl JsonFile {
    pub fn new() -> Self {
        JsonFile {  }
    }

    fn read(&self, path: String) -> Result<String, String>  {
        match File::open(file_path) {
            Ok(file) => {
                return Ok(BufReader::new(file));
            },
            Err(err) => {
                warn!("JSON.reader() | error: {:?}.",err);
                return Err(err.to_string());
            }
        }
    }

    pub fn content<T, P>(&self, file_path: P) -> Result<T, String>
    where
        P: AsRef<Path>,
        T: for<'de> Deserialize<'de>
    {
        match self.read(path) {
            Ok(content) => {
                match serde_json::from_str(content) {
                    Ok(object) => { Ok(object) },
                    Err(err) => {
                        warn!("JSON.from_file() | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }        
            },
            Err(err) => {err},
        };
    }
}
