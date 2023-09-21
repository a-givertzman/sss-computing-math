use serde::Deserialize;

use crate::equivalent_beam::{system_of_units::{Newton, Meters}, location::Location};

#[derive(Deserialize, Debug)]
pub struct Force {
    value: Newton,
    location: Location,
    length: Meters,
}