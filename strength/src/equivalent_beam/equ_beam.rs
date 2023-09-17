use crate::pre_data::{cross_section_properties::cross_sections::CrossSections, ship::ship::Ship};





pub struct EquBeam {
    cross_sections: Result<CrossSections, String>,
    ship: Result<Ship, String>,
}

impl EquBeam {
    pub fn new(cross_sections: Result<CrossSections, String>, ship: Result<Ship, String>) -> Self {
        EquBeam { cross_sections, ship }
    }

    pub fn solve(&self) {
        todo!()
    }
}