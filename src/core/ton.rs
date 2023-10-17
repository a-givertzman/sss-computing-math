use serde::Deserialize;

use super::newton::Newton;


#[derive(Deserialize, Debug)]
pub struct Ton(pub f64);


impl From<Newton> for Ton {
    fn from(value: Newton) -> Self {
        Ton((value.0 / 9.81) / 1000.0)
    }
}
