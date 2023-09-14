use super::cross_section::CrossSection;



pub struct EquBeam {
    cross_sections: Vec<CrossSection>,
}

impl EquBeam {
    fn cross_section(&self, id: usize) -> &CrossSection {
        &self.cross_sections[id]
    }
}