use super::cross_section::CrossSection;
use std::collections::HashMap;



pub struct EquBeam {
    cross_sections: HashMap<i32, CrossSection>,
}

impl EquBeam {
    fn cross_section(&self, id: i32) -> Option<&CrossSection> {
        self.cross_sections.get(&id)
    }

    fn new(cross_sections: HashMap<i32, CrossSection>) -> Self {
        EquBeam { cross_sections }
    }
}