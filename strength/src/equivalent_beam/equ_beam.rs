use log::warn;
use crate::pre_data::{cross_section_properties::cross_sections::CrossSections, ship::loads::{type_load::TypeLoad, load::Load}};
use super::spatium::Spatium;





pub struct EquBeam {
    cross_sections: Result<CrossSections, String>,
    load: Result<Load, String>,
}

impl EquBeam {
    pub fn new(cross_sections: Result<CrossSections, String>, load: Result<Load, String>) -> Self {
        EquBeam { cross_sections, load }
    }

    pub fn spatiums(&self, type_load: TypeLoad) -> Result<Vec<Spatium>, String> {
        match &self.load {
            Ok(load) => { load.spatiums(type_load) },
            Err(err) => {
                warn!("EquBeam::spatiums() | error: {:?}.", err);
                return Err(err.to_string());
            }
        }
    }
}