use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Location { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }


}

