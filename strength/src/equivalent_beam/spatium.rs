#[derive(Debug)]
pub struct Spatium {
    start_coord: f64,
    end_coord: f64,
    start_value: f64,
    end_value: f64,

}

impl Spatium {
    pub fn new(start_coord: f64,
        end_coord: f64,
        start_value: f64,
        end_value: f64,) -> Self {
        Spatium { start_coord, end_coord, start_value, end_value }
    }


}