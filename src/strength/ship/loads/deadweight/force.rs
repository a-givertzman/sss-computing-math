use serde::Deserialize;
use crate::core::{newton::Newton, location::Location};

#[derive(Deserialize, Debug)]
pub struct Force {
    value: Newton,
    location: Location,
    length: f64,
}