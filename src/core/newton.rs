use serde::Deserialize;

use super::ton::Ton;

#[derive(Deserialize, Debug)]
#[serde(from = "f64")]
pub struct Newton(pub f64);

impl From<f64> for Newton {
    fn from(value: f64) -> Self {
        let newton = value * 1000.0 * 9.81;
        Newton(newton)
    }
}

impl From<Ton> for Newton {
    fn from(ton: Ton) -> Newton {
        let newton = ton.0 * 1000.0 * 9.81;
        Newton(newton)
    }
}
