use log::warn;

use crate::{ship::{cross_section_properties::cross_sections::CrossSections, loads::{load::Load, type_load::TypeLoad}}, core::diagram::Diagram};



pub struct EquBeam {
    cross_sections: Result<CrossSections, String>,
    load: Result<Load, String>,
}

impl EquBeam {
    pub fn new(cross_sections: Result<CrossSections, String>, load: Result<Load, String>) -> Self {
        EquBeam { cross_sections, load }
    }

    pub fn diagram(&self, type_load: TypeLoad) -> Result<Diagram, String> {
        match &self.load {
            Ok(load) => {
                match load.spatiums(type_load) {
                    Ok(spatiums) => {
                        Ok(Diagram::new(spatiums, type_load))
                    },
                    Err(err) => {
                        warn!("EquBeam::spatiums() | error: {:?}.", err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("EquBeam::spatiums() | error: {:?}.", err);
                return Err(err.to_string());
            }
        }
    }
}