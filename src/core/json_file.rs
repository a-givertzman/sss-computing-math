use std::{io::BufReader, fs::File};
use log::warn;

pub struct JsonFile {
    path: String,
}


///
/// Reads json file
impl JsonFile {
    pub fn new(path: String) -> Self {
        JsonFile { path }
    }

    ///
    /// Return content json file.
    pub fn content(&self) -> Result<BufReader<File>, String>
    {
        match File::open(&self.path) {
            Ok(file) => {
                Ok(BufReader::new(file))

            },
            Err(err) => {
                warn!("JsonFile.content() | error {:?}", err);
                return Err(err.to_string());
            }
        }
    }
}
