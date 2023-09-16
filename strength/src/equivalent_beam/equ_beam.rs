use super::pre_data::{pre_data::PreData, ship::ship::Ship};



pub struct EquBeam {
    pre_data: PreData,
    ship: Result<Ship, String>
}

impl EquBeam {
    pub fn new(pre_data: PreData, ship: Result<Ship, String>) -> Self {
        EquBeam { pre_data, ship }
    }

    pub fn solve(&self) {
        let cross_sections = self.pre_data.cross_sections();
    }
}