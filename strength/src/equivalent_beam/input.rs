use std::path::Path;
use csv::Reader;
use super::cross_section::CrossSection;

pub struct Input {
    file_path: String,
}


impl Input {
    pub fn new(file_path: String) -> Input {
        Input { file_path }
    }

    pub fn run(&self) {
        let file_path = Path::new(&self.file_path);
        let mut reader = Reader::from_path(&self.file_path).unwrap();
        for result in reader.records() {
            let record = result.unwrap();
            println!("{:?}", record);
        }

    }
}