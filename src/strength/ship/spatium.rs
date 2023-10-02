#[derive(Debug)]
pub struct Spatium {
    pub x1: f64,
    pub x2: f64,
    pub f_x1: f64,
    pub f_x2: f64,

}

impl Spatium {
    pub fn new(x1: f64,
        x2: f64,
        f_x1: f64,
        f_x2: f64,) -> Self {
        Spatium { x1, x2, f_x1, f_x2 }
    }

    pub fn square(&self) -> f64 {
        ((self.f_x1 + self.f_x2) / 2.0) * (self.x2 - self.x1)
    }
}