#[derive(Clone, Copy)]
pub struct Point2D {
    pub y: f64,
    pub z: f64,
}


impl Point2D {
    pub fn new(y: f64, z: f64) -> Self {
        Point2D { y, z }
    }
}