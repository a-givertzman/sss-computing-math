use csv::Reader;
use csv::Error as CSVError;
use log::trace;
use log::warn;
use std::fs::File;



pub struct Input<'a> {
    file_path: &'a str,
}


impl<'a> Input<'_> {
    pub fn new(file_path: &str) -> Input {
        Input { file_path }
    }

    pub fn run(&self) -> Result<Reader<File>, CSVError> {
        match Reader::from_path(self.file_path) {
            Ok(reader) => {
                trace!("reader: {:?}", reader);
                Ok(reader)
            },
            Err(err) => {
                warn!("Input.run | error: {:?}",err);
                Err(err)
            }
        }   
    }
}