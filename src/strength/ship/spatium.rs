#[derive(Debug)]
pub struct Spatium {
    pub start_coord: f64,
    pub end_coord: f64,
    pub start_value: f64,
    pub end_value: f64,

}

impl Spatium {
    pub fn new(start_coord: f64,
        end_coord: f64,
        start_value: f64,
        end_value: f64,) -> Self {
        Spatium { start_coord, end_coord, start_value, end_value }
    }

    pub fn square(&self) -> f64 {
        ((self.start_value + self.end_value) / 2.0) * (self.end_coord - self.start_coord)
    }
}