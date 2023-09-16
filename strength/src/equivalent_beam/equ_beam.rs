use super::pre_data::{ship::ship::Ship, cross_section::cross_sections::CrossSections};



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