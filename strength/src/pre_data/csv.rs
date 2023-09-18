use csv::Reader;
use csv::Error as CSVError;
use log::trace;
use log::warn;
use std::fs::File;

pub trait FromCSV {
    fn from_csv(file_path: String) -> Result<Self, String>
    where Self: Sized;
}

pub struct CSV<'a> {
    file_path: &'a str,
}


impl<'a> CSV<'_> {
    pub fn new(file_path: &str) -> CSV {
        CSV { file_path }
    }

    pub fn parser(&self) -> Result<Reader<File>, CSVError> {
        match Reader::from_path(self.file_path) {
            Ok(reader) => {
                trace!("reader: {:?}", reader);
                Ok(reader)
            },
            Err(err) => {
                warn!("CSV.run | error: {:?}",err);
                Err(err)
            }
        }
    }
}