use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Newton(f64);

#[derive(Deserialize, Debug)]
struct Tons(f64);


impl From<f64> for Newton {
    fn from(value: f64) -> Self {
        let newton = value * 1000.0 * 9.81;
        Newton(newton)
    }
}


impl From<Tons> for Newton {
    fn from(tonn: Tons) -> Newton {
        let newton = tonn.0 * 1000.0 * 9.81;
        Newton(newton)
    }
}

impl From<Newton> for Tons {
    fn from(value: Newton) -> Self {
        Tons((value.0 / 9.81) / 1000.0)
    }
}


#[derive(Deserialize, Debug)]
pub struct Meters(f64);
