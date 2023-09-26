use log::warn;

use crate::{ship::{cross_section_properties::cross_sections::CrossSections, loads::load::Load}, core::output::{output::Output, type_output::TypeOutput}};




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
                match load.spatiums(type_output) {
                    Ok(spatiums) => {
                        Ok(Output::new(spatiums, type_output))
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