use csv::Reader;
use csv::Error as CSVError;
use std::collections::HashMap;
use super::cross_section::CrossSection;


pub struct Input {
    file_path: String,
}


impl Input {
    pub fn new(file_path: String) -> Input {
        Input { file_path }
    }

    pub fn run(&self) -> Result<HashMap<i32, CrossSection>, CSVError>{
        let mut cross_sections = HashMap::new();
        match Reader::from_path(&self.file_path) {
            Ok(reader) => {
                debug!("lines: {:?} read", reader.len());
                trace!("reader: {:?}", reader);
                Ok(reader)
            },
            Err(err) => {
                warn!("Input.run | error: {:?}",err)
                Err(err)
            }
        }
    }
}