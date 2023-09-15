use std::collections::HashMap;
use csv::Error as CSVError;
use super::{cross_section::CrossSection, input::Input};


pub struct EquBeam {
    input: Input,
    cross_sections: HashMap<i32, CrossSection>,
}

impl EquBeam {
    fn cross_section(&self, id: i32) -> Option<&CrossSection> {
        self.cross_sections.get(&id)
    }

    pub fn new(input: Input, cross_sections: HashMap<i32, CrossSection>) -> Self {
        EquBeam { input, cross_sections }
    }

    pub fn run(&mut self) -> Result<(), CSVError>{
        self.cross_sections = self.input.run()?;
        Ok(())
    }
}