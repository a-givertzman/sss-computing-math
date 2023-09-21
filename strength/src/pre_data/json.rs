use std::{io::BufReader, fs::File};

use log::warn;

pub struct JSON {
    file_path: String,
}

impl JSON {
    pub fn new(file_path: String) -> Self {
        JSON { file_path }
    }
    pub fn reader(&self) -> Result<BufReader<File>, String>  {
        match File::open(&self.file_path) {
            Ok(file) => {
                return Ok(BufReader::new(file));
            },
            Err(err) => {
                warn!("Ship::from_json_file | error: {:?}.",err);
                return Err(err.to_string());
            }
        }
    }
}
