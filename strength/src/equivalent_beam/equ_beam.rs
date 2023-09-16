use super::pre_data::pre_data::PreData;



pub struct EquBeam {
    pre_data: PreData,
}

impl EquBeam {
    pub fn new(pre_data: PreData) -> Self {
        EquBeam { pre_data }
    }

    pub fn solve(&self) {
        let cross_sections = self.pre_data.cross_sections();
        let ship = self.pre_data.ship();
    }
}