#[derive(Debug)]
pub struct Spatium {
    id: String,
    value: f64,
    length: f64
}

impl Spatium {
    pub fn new(id: String, value: f64, length: f64) -> Self {
        Spatium { id, value, length }
    }


}