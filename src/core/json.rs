use std::{io::BufReader, fs::File, path::Path};
use serde::Deserialize;
use log::warn;

pub struct JSON { }

impl JSON {
    pub fn new() -> Self {
        JSON {  }
    }

    fn reader<P: AsRef<Path>>(&self, file_path: P) -> Result<BufReader<File>, String>  {
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

    pub fn from_file<T, P>(&self, file_path: P) -> Result<T, String>
    where
        P: AsRef<Path>,
        T: for<'de> Deserialize<'de>
    {
        let reader = self.reader(file_path)?;
        match serde_json::from_reader(reader) {
            Ok(object) => { Ok(object) },
            Err(err) => {
                warn!("JSON.from_file() | error: {:?}.",err);
                return Err(err.to_string());
            }
        }
    }
}
