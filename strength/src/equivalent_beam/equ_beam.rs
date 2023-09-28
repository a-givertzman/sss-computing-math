use log::warn;

use crate::{ship::{cross_section_properties::cross_sections::CrossSections, loads::load::Load}, output::{type_output::TypeOutput, output::Output}};




pub struct EquBeam {
    cross_sections: Result<CrossSections, String>,
    load: Result<Load, String>,
}

impl EquBeam {
    pub fn new(cross_sections: Result<CrossSections, String>, load: Result<Load, String>) -> Self {
        EquBeam { cross_sections, load }
    }

    pub fn output(&self, type_output: TypeOutput) -> Result<Output, String> {
        match &self.load {
            Ok(load) => {
                let output = load.spatiums(type_output);
                Ok(output)
            },
            Err(err) => {
                warn!("EquBeam::spatiums() | error: {:?}.", err);
                return Err(err.to_string());
            }
        }
    }
}