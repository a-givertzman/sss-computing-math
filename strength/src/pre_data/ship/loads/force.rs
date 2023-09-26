use serde::Deserialize;
use crate::core::{system_of_units::Newton, location::Location};

#[derive(Deserialize, Debug)]
pub struct Force {
    value: Newton,
    location: Location,
    length: f64,
}