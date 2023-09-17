use crate::equivalent_beam::{system_of_units::{Newton, Meters}, location::Location};


struct Force {
    value: Newton,
    location: Location,
    length: Meters,
}