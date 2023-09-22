#[derive(Debug)]
pub struct Spatium {
    id: i64,
    value: f64,
    length: f64
}

impl Spatium {
    pub fn new(id: i64, value: f64, length: f64) -> Self {
        Spatium { id, value, length }
    }


}